([
  (line_comment)
  (block_comment)
  (comment_environment)
] @content
  (#set! "language" "comment"))

(pycode_environment
  code: (source_code) @content
  (#set! "language" "python"))

(sagesilent_environment
  code: (source_code) @content
  (#set! "language" "python"))

(sageblock_environment
  code: (source_code) @content
  (#set! "language" "python"))

(minted_environment
  (begin
    language:
      (curly_group_text
        (text) @language))
  (source_code) @content)

((generic_environment
  (begin
    name:
      (curly_group_text
        (text) @_env))) @content
  (#set! "language" "c")
  (#any-of? @_env "asy" "asydef"))

(generic_command
  command: (_) @_command
  arg: (curly_group (_) @content)
  (#set! "language" "python")
  (#eq? @_command "\\sage"))

(generic_command
  command: (_) @_command
  arg: (curly_group (_) @content)
  (#set! "language" "lua")
  (#any-of? @_command "\\directlua" "\\latelua"))
