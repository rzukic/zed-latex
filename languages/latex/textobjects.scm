; Class textobject

(part
  text: (_)
  .
  (_)* @class.inside ) @class.around

(chapter
  text: (_)
  .
  (_)* @class.inside ) @class.around

(section
  text: (_)
  .
  (_)* @class.inside ) @class.around

(subsection
  text: (_)
  .
  (_)* @class.inside ) @class.around

(subsubsection
  text: (_)
  .
  (_)* @class.inside ) @class.around

; I don't think these are useful over { } vim motions
; (paragraph
;   text: (_)
;   .
;   (_)* @class.inside ) @class.around
;
; (subparagraph
;   text: (_)
;   .
;   (_)* @class.inside ) @class.around


; Function textobject

(generic_environment
  begin: (_)
; (begin
;     name: (curly_group_text
;     text: (_) @_not_document)
;     (#not-eq? @_not_document "document"))
  .
  (_)* @function.inside
  .
  end: (_)) @function.around

(math_environment
  begin: (_)
  .
  _* @function.inside
  .
  end: (_)) @function.around

(displayed_equation
  "$$"
  .
  _* @function.inside
  .
  "$$") @function.around

(displayed_equation
  "\\["
  .
  _* @function.inside
  .
  "\\]") @function.around

; Comment textobject

((line_comment)+ @comment.around)

(block_comment
  comment: _ @comment.inside) @comment.around
