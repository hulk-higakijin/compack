/// ZSH shell integration script
pub const ZSH_INTEGRATION: &str = r#"# compack zsh integration
_compack_completion_or_space() {
    # First insert the space
    zle self-insert

    # Get the command (first word before space)
    local cmd="${LBUFFER%% *}"

    # Only trigger if we have exactly one word and just typed a space
    if [[ "$LBUFFER" == "$cmd " && "$cmd" != "" ]]; then
        # Check if this command has subcommands defined
        local candidates=$(compack query "$cmd" 2>/dev/null)

        if [[ -n "$candidates" ]]; then
            # Add a special option to run without subcommand
            local all_options="[Run without subcommand]"$'\n'"$candidates"
            
            # Command is defined, open fzf
            local selected=$(echo "$all_options" | fzf --height 40% --reverse --prompt="$cmd > " --bind=tab:down,shift-tab:up)

            if [[ -n "$selected" ]]; then
                if [[ "$selected" == "[Run without subcommand]" ]]; then
                    # User wants to run the command without subcommand
                    LBUFFER="${cmd}"
                    # Execute the command immediately
                    zle accept-line
                else
                    # User selected a subcommand
                    LBUFFER="${cmd} ${selected}"
                    # Execute the command immediately
                    zle accept-line
                fi
            else
                # If nothing selected (ESC pressed), LBUFFER stays as "${cmd} " with the space
                # Redraw the prompt to show the current buffer
                zle reset-prompt
            fi
        fi
        # If no candidates (command not defined), just keep the space as normal input
    fi
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
