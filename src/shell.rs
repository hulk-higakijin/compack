/// ZSH shell integration script
pub const ZSH_INTEGRATION: &str = r#"# compack zsh integration
_compack_completion_or_space() {
    # First insert the space
    zle self-insert

    # Get the command (first word before space)
    local cmd="${LBUFFER%% *}"

    # Early return: Only trigger if we have exactly one word and just typed a space
    [[ "$LBUFFER" != "$cmd " || -z "$cmd" ]] && return

    # Check if this command has subcommands defined
    local candidates=$(compack query "$cmd" 2>/dev/null)
    
    # Early return: No candidates means command not defined, keep space as normal input
    [[ -z "$candidates" ]] && return

    # Add a special option to run without subcommand
    local all_options="[Run without subcommand]"$'\n'"$candidates"
    
    # Open fzf for selection
    local selected=$(echo "$all_options" | fzf --height 40% --reverse --prompt="$cmd > " --bind=tab:down,shift-tab:up)

    # Handle ESC pressed (nothing selected)
    if [[ -z "$selected" ]]; then
        zle reset-prompt
        return
    fi

    # Set the command line based on selection
    if [[ "$selected" == "[Run without subcommand]" ]]; then
        LBUFFER="${cmd}"
    else
        LBUFFER="${cmd} ${selected}"
        
        # Check if this subcommand has nested subcommands
        local nested_candidates=$(compack query "$cmd $selected" 2>/dev/null)
        
        # If nested subcommands exist, trigger another selection
        if [[ -n "$nested_candidates" ]]; then
            # Add option to run without nested subcommand
            local nested_options="[Run without nested subcommand]"$'\n'"$nested_candidates"
            
            # Open fzf for nested selection
            local nested_selected=$(echo "$nested_options" | fzf --height 40% --reverse --prompt="$cmd $selected > " --bind=tab:down,shift-tab:up)
            
            # Handle ESC pressed (nothing selected)
            if [[ -z "$nested_selected" ]]; then
                zle reset-prompt
                return
            fi
            
            # Set the command line based on nested selection
            if [[ "$nested_selected" != "[Run without nested subcommand]" ]]; then
                LBUFFER="${cmd} ${selected} ${nested_selected}"
            fi
        fi
    fi
    
    # Execute the command
    zle accept-line
}

zle -N _compack_completion_or_space
bindkey ' ' _compack_completion_or_space
"#;

pub enum Shell {
    Zsh,
}

impl Shell {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "zsh" => Ok(Shell::Zsh),
            other => Err(format!("Shell '{}' is not supported yet. Currently supported: zsh", other)),
        }
    }

    pub fn integration_script(&self) -> &'static str {
        match self {
            Shell::Zsh => ZSH_INTEGRATION,
        }
    }
}
