(doctype) @constant
(entity) @constant

(erroneous_end_tag_name) @tag
(erroneous_cf_end_tag_name) @tag
(attribute_name) @attribute
(cf_attribute_name) @attribute
(attribute_value) @string
(start_tag) @tag
(end_tag) @tag
(self_closing_tag) @tag
(cf_selfclose_tag) @tag
(cf_start_tag_with_selfclose) @tag
(cf_output_tag) @tag
(cf_script_tag) @tag
(cf_start_tag) @tag
(cf_end_tag) @tag
(cf_if_tag) @tag
(cf_query_tag) @tag
(cf_else_tag) @tag
(cf_elseif_tag) @tag
(cf_return_tag) @tag
(cf_xml_tag) @tag

(tag_name) @tag
(cf_tag_name) @tag

; Variables
;----------

(identifier) @variable

; CFML scopes (variables, session, etc.)
(cf_scope_identifier) @variable.special

; Properties
;-----------

(property_identifier) @property

(shorthand_property_identifier) @property

; Function and method definitions
;--------------------------------

(function_expression
  name: (identifier) @function)

(function_declaration
  name: (identifier) @function)

(function_declaration
  (access_type) @keyword)

(function_declaration
  (return_type) @type)

(method_definition
  name: [
    (property_identifier)
    (private_property_identifier)
  ] @function.method)

(method_definition
  name: (property_identifier) @constructor
  (#eq? @constructor "constructor"))

(pair
  key: (property_identifier) @function.method
  value: (function_expression))

(pair
  key: (property_identifier) @function.method
  value: (arrow_function))

(array) @variable

(cf_set_tag) @tag

(assignment_expression
  left: (member_expression
    property: (property_identifier) @function.method)
  right: (arrow_function))

(assignment_expression
  left: (member_expression
    property: (property_identifier) @function.method)
  right: (function_expression))

(variable_declarator
  name: (identifier) @function
  value: (arrow_function))

(variable_declarator
  name: (identifier) @function
  value: (function_expression))

(assignment_expression
  left: (identifier) @function
  right: (arrow_function))

(assignment_expression
  left: (identifier) @function
  right: (function_expression))

; Function and method calls
;--------------------------

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: [
      (property_identifier)
      (private_property_identifier)
    ] @function.method))

; Literals
;---------

((identifier) @variable.special
  (#eq? @variable.special "self"))

(this) @variable.special
(super) @variable.special

(cf_var) @keyword

[
  (true)
  (false)
] @boolean

[
  (null)
  (undefined)
] @constant.special

[
  (comment)
  (cf_comment)
] @comment

((comment) @comment.doc
  (#lua-match? @comment.doc "^/[*][*][^*].*[*]/$"))

(hash_single) @keyword
(string) @string
(text) @string
(hash_empty) @punctuation.special

(regex_pattern) @string.regex
(regex_flags) @string.special

(regex
  "/" @punctuation.bracket)

(number) @number

(hash_expression
  "#" @punctuation.special)

(unary_operator) @operator

((identifier) @number
  (#any-of? @number "NaN" "Infinity"))

; Punctuation
;------------
[
  ";"
  "."
  ","
  ":"
  (optional_chain)
  (static_chain)
] @punctuation.delimiter

(binary_expression
  "/" @operator)

(ternary_expression
  [
    "?"
    ":"
  ] @operator)

(elvis_expression
  "?:" @operator)

[
  "-"
  "--"
  "-="
  "+"
  "++"
  "+="
  "*"
  "*="
  "**"
  "**="
  "/"
  "/="
  "%"
  "%="
  "<"
  "<<="
  "="
  "=="
  "==="
  "!"
  "!="
  "!=="
  "=>"
  ">"
  ">>="
  ">>>="
  "~"
  "^"
  "&"
  "|"
  "^="
  "&="
  "|="
  "&&"
  (logical_or)
  "||"
  "??"
  "&&="
  "||="
  "??="
] @operator

[
  "var"
  "let"
  "const"
  "function"
  "new"
  "return"
  "if"
  "else"
  "for"
  "while"
  "do"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "try"
  "catch"
  "finally"
  "throw"
  "in"
  "of"
  "instanceof"
  "static"
  "export"
  "yield"
  "with"
] @keyword

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
  "</"
] @punctuation.bracket
