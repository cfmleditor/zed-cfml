((script_element
  (raw_text) @injection.content)
 (#set! injection.language "javascript"))

((style_element
  (raw_text) @injection.content)
 (#set! injection.language "css"))

((cf_script_tag
  (cf_script_content) @injection.content)
 (#set! injection.language "cfscript"))
 
((cf_script_content) @injection.content (#set! injection.language "CFML (Script)"))

((cf_query_tag
  (cf_query_content) @injection.content)
 (#set! injection.language "cfquery"))

((cf_query_content) @injection.content (#set! injection.language "CFML (Query)"))