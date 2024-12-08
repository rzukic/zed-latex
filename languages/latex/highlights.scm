; General syntax
(command_name) @function

(caption
  command: _ @function)

; Turn spelling on for text
;(text) @none

; \text, \intertext, \shortintertext, ...
(text_mode
  command: _ @function
  content: (curly_group
    (_) @none))

; Variables, parameters
(placeholder) @variable

(key_value_pair
  key: (_) @variable.parameter
  value: (_))

; Does not currently exist in the LaTeX grammar, maybe to come?:
;(curly_group_spec
;  (text) @variable.parameter)
;

(brack_group_argc) @attribute

[
  (operator)
  "="
  "_"
  "^"
] @operator

"\\item" @punctuation.list_marker

; Does not currently exist in the LaTeX grammar, maybe to come?:
;(delimiter) @punctuation.delimiter

(math_delimiter
  left_command: _ @punctuation.bracket
  left_delimiter: _ @punctuation.bracket
  right_command: _ @punctuation.bracket
  right_delimiter: _ @punctuation.bracket)

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket ; "(" ")" has no syntactical meaning in LaTeX

; General environments
(begin
  command: _ @keyword.module
  name: (curly_group_text
    (text) @type))

(end
  command: _ @keyword.module
  name: (curly_group_text
    (text) @type))

; Definitions and references
(new_command_definition
  command: _ @function.macro)

(old_command_definition
  command: _ @function.macro)

(let_command_definition
  command: _ @function.macro)

(environment_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(theorem_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant)
  title: (curly_group (_) @title)?
  counter: (brack_group_text (_) @constant)?)

(paired_delimiter_definition
  command: _ @function.macro
  declaration: (curly_group_command_name
    (_) @function))

(label_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(label_reference_range
  command: _ @function.macro
  from: (curly_group_text
    (_) @constant)
  to: (curly_group_text
    (_) @constant))

(label_reference
  command: _ @function.macro
  names: (curly_group_text_list
    (_) @constant))

(label_number
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant)
  number: (_) @constant)

(citation
  command: _ @function.macro
  keys: (curly_group_text_list) @constant)

(label_definition
  name: (curly_group_text
    (text
      word: (operator "-") @constant)))

(label_definition
  name: (curly_group_text
    (text
      word: (subscript "_" @constant))))

(label_definition
  name: (curly_group_text
    (text
      word: (superscript "^" @constant))))

(label_reference
  names: (curly_group_text_list
    (text
      word: (operator "-") @constant)))

(label_reference
  names: (curly_group_text_list
    (text
      word: (subscript "_" @constant))))

(label_reference
  names: (curly_group_text_list
    (text
      word: (superscript "^" @constant))))

; Does not currently exist in the LaTeX grammar, maybe to come?:
;(hyperlink
  ;command: (_) @function
  ;uri: (_) @link_uri)
((generic_command
  command: (command_name) @_name @function
  .
  arg:
    (curly_group
      (_) @link_uri))
  (#any-of? @_name "\\url" "\\href"))

(glossary_entry_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(glossary_entry_reference
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(acronym_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(acronym_reference
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(color_definition
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

(color_reference
  command: _ @function.macro
  name: (curly_group_text
    (_) @constant))

; Sectioning
(title_declaration
  command: _ @keyword.module
  options: (brack_group
    (_) @title.1)?
  text: (curly_group
    (_) @title.1))

(author_declaration
  command: _ @keyword.module
  authors: (curly_group_author_list
    (author)+ @title.1))

(chapter
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.2)?
  text: (curly_group
    (_) @title.2))

(part
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.2)?
  text: (curly_group
    (_) @title.2))

(section
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.3)?
  text: (curly_group
    (_) @title.3))

(subsection
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.4)?
  text: (curly_group
    (_) @title.4))

(subsubsection
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.5)?
  text: (curly_group
    (_) @title.5))

(paragraph
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.6)?
  text: (curly_group
    (_) @title.6))

(subparagraph
  command: _ @keyword.module
  toc: (brack_group
    (_) @title.6)?
  text: (curly_group
    (_) @title.6))

;; Beamer frames
(generic_environment
  (begin
    name: (curly_group_text
      (text) @constant)
    (#any-of? @constant "frame"))
  .
  (curly_group
    (_) @title))

((generic_command
  command: (command_name) @_name @function
  arg: (curly_group
    (_) @title))
  (#eq? @_name "\\frametitle"))

((generic_command
  command: (command_name) @_name @function
  arg: (curly_group
    (_) @emphasis))
  (#any-of? @_name "\\emph" "\\textit" "\\mathit"))

((generic_command
  command: (command_name) @_name @function
  arg: (curly_group
    (_) @emphasis.strong))
  (#any-of? @_name "\\textbf" "\\mathbf"))

;; File inclusion commands
(class_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string))

(package_include
  command: _ @keyword.import
  paths: (curly_group_path_list) @string)

(latex_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string.special.path))

(verbatim_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string.special.path))

(import_include
  command: _ @keyword.import
  directory: (curly_group_path (_) @string.special.path)
  file: (curly_group_path (_) @string.special.path))

(bibstyle_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string))

(bibtex_include
  command: _ @keyword.import
  paths: (curly_group_path_list) @string.special.path)

(biblatex_include
  "\\addbibresource" @keyword.import
  glob: (curly_group_glob_pattern (_) @string.regexp))

(graphics_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string.special.path))

(svg_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string.special.path))

(inkscape_include
  command: _ @keyword.import
  path: (curly_group_path (_) @string.special.path))

(tikz_library_import
  command: _ @keyword.import
  paths: (curly_group_path_list) @string)

; Math
[
  (displayed_equation)
  (inline_formula)
] @number

(math_environment
  (_) @number)

;; Comments
[
  (line_comment)
  (block_comment)
  (comment_environment)
] @comment

((line_comment) @keyword.directive
  (#match? @keyword.directive "^%% !TeX"))

((line_comment) @keyword.directive
  (#match? @keyword.directive "^%%&"))
