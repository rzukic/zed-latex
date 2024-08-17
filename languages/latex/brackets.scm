; \left and \right delimiters
(math_delimiter
  left_command: _
  left_delimiter: _ @open
  right_command: _
  right_delimiter: _ @close)

; math
(inline_formula
  "$" @open
  "$" @close)
(inline_formula
  "\\(" @open
  "\\)" @close)
(displayed_equation
  "$$" @open
  "$$" @close)
(displayed_equation
  "\\[" @open
  "\\]" @close)

; curly brackets
(curly_group "{" @open "}" @close)
(curly_group_text "{" @open "}" @close)
(curly_group_text_list "{" @open "}" @close)
(curly_group_path "{" @open "}" @close)
(curly_group_path_list "{" @open "}" @close)
(curly_group_command_name "{" @open "}" @close)
(curly_group_key_value "{" @open "}" @close)
(curly_group_glob_pattern "{" @open "}" @close)
(curly_group_impl "{" @open "}" @close)
(curly_group_author_list "{" @open "}" @close)

; square brackets
(brack_group "[" @open "]" @close)
(brack_group_text "[" @open "]" @close)
(brack_group_argc "[" @open "]" @close)
(brack_group_key_value "[" @open "]" @close)

; environments
(generic_environment
  begin: _ @open
  end: _ @close)
(comment_environment
  begin: _ @open
  end: _ @close)
(verbatim_environment
  begin: _ @open
  end: _ @close)
(listing_environment
  begin: _ @open
  end: _ @close)
(minted_environment
  begin: _ @open
  end: _ @close)
(pycode_environment
  begin: _ @open
  end: _ @close)
(sagesilent_environment
  begin: _ @open
  end: _ @close)
(sageblock_environment
  begin: _ @open
  end: _ @close)
(math_environment
  begin: _ @open
  end: _ @close)

