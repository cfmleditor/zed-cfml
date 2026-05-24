; Parentheses (subqueries, IN lists, function args)
(parenthesized_query_node) @indent
(")") @outdent

; CF tags within queries
(cf_if_tag) @indent
(cf_tag (cf_end_tag)) @indent

; Indent/outdent for braces (cfscript expressions)
("{") @indent
("}") @outdent
