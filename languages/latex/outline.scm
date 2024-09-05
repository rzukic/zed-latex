; For each declaration, prioritize using the optional toc entry instead of the
; curly group contents, which may contain non-text content

; FUTURE: when possible consider processing things like \texorpdfstring{.}{.}
; to only take pdfstring


; CHAPTER DECLARATIONS

(chapter
  command: _ @context
  !toc
  text: _ @name) @item

(chapter
  command: _ @context
  toc: _ @name) @item

; PART DECLARATIONS

(part
  command: _ @context
  toc: _ @name) @item

(part
  command: _ @context
  !toc
  text: _ @name) @item

; SECTION DECLARATIONS

(section
  command: _ @context
  toc: _ @name) @item

(section
  command: _ @context
  !toc
  text: _ @name) @item

; SUBSECTION DECLARATIONS

(subsection
  command: _ @context
  toc: _ @name) @item

(subsection
  command: _ @context
  !toc
  text: _ @name) @item

; SUBSUBSECTION DECLARATIONS

(subsubsection
  command: _ @context
  toc: _ @name) @item

(subsubsection
  command: _ @context
  !toc
  text: _ @name) @item

