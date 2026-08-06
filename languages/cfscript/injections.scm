;; SQL inside inline queries, e.g. queryExecute("SELECT ...")
((query_expression
  (query_text) @injection.content)
 (#set! injection.language "CFML (Query)"))

;; Tag markup inside ``` template blocks
((cfml_template
  (cfml_template_content) @injection.content)
 (#set! injection.language "CFML (Tag)"))
