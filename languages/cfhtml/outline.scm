; Function definitions
(function_declaration
  name: (identifier) @name) @item

(function_expression
  name: (identifier) @name) @item

(method_definition
  name: (property_identifier) @name) @item

; cffunction tag
(cf_tag
  (cf_start_tag
    (cf_tag_name) @_cffunction
    (tag_attributes
      (attribute
        (attribute_name) @_name
        (_ (attribute_value) @name))))
  (#eq? @_cffunction "function")
  (#eq? @_name "name")) @item
