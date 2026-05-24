; Parentheses (subqueries, IN lists, function args)
(parenthesized_query_node) @indent

; CF tags within queries
(cf_if_tag) @indent
(cf_tag
  (cf_end_tag)) @outdent

; Outdent closing delimiter
(")") @outdent
