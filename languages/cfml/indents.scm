; Indent inside block-level CF tags
(cf_if_tag) @indent
(cf_output_tag) @indent
(cf_function_tag) @indent
(cf_query_tag) @indent
(cf_script_tag) @indent
(cf_xml_tag) @indent
(cf_savecontent_tag) @indent
(cf_tag (cf_end_tag)) @indent

; Indent inside HTML elements
(element) @indent
(self_closing_tag) @outdent

; Blocks
(statement_block) @indent
("}" @end) @outdent

; Arrays and objects
(array) @indent
(object) @indent
("]" @end) @outdent

; Parentheses
(arguments) @indent
(formal_parameters) @indent
(parenthesized_expression) @indent
(")" @end) @outdent
