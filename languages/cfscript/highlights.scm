; Variables
;----------

(identifier) @variable

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
(method_definition
  name: (property_identifier) @function.method)

(pair
  key: (property_identifier) @function.method
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (member_expression
    property: (property_identifier) @function.method)
  right: [(function_expression) (arrow_function)])

(variable_declarator
  name: (identifier) @function
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (identifier) @function
  right: [(function_expression) (arrow_function)])

; Function and method calls
;--------------------------

(call_expression
  function: (identifier) @function.builtin
  (#match? @function.builtin "(?i)^(abort|abs|acos|addSOAPRequestHeader|addSOAPResponseHeader|arrayAppend|arrayAvg|arrayClear|arrayContains|arrayContainsNoCase|arrayDelete|arrayDeleteAt|arrayDeleteNoCase|arrayEach|arrayEvery|arrayFilter|arrayFind|arrayFindAll|arrayFindAllNoCase|arrayFindNoCase|arrayFirst|arrayGetMetadata|arrayIndexExists|arrayInsertAt|arrayIsDefined|arrayIsEmpty|arrayLast|arrayLen|arrayMap|arrayMax|arrayMedian|arrayMerge|arrayMid|arrayMin|arrayNew|arrayPop|arrayPrepend|arrayPush|arrayReduce|arrayReduceRight|arrayResize|arrayReverse|arraySet|arraySlice|arraySort|arraySplice|arraySum|arraySwap|arrayToList|arrayToStruct|asc|asin|atn|beat|binaryDecode|binaryEncode|bitAnd|bitMaskClear|bitMaskRead|bitMaskSet|bitNot|bitOr|bitSHLN|bitSHRN|bitXor|booleanFormat|cacheCount|cacheGet|cacheGetAll|cacheGetAllIds|cacheGetMetadata|cacheGetProperties|cacheGetSession|cacheIdExists|cachePut|cacheRegionExists|cacheRegionNew|cacheRegionRemove|cacheRemove|cacheRemoveAll|cacheSetProperties|callStackDump|callStackGet|canonicalize|ceiling|cfusion_decrypt|cfusion_encrypt|charsetDecode|charsetEncode|chr|cJustify|collectionEach|collectionEvery|collectionFilter|collectionMap|collectionReduce|collectionSome|compare|compareNoCase|cos|createDate|createDateTime|createDynamicProxy|createGUID|createObject|createODBCDate|createODBCDateTime|createODBCTime|createTime|createTimeSpan|createUniqueId|createUUID|csrfGenerateToken|csrfVerifyToken|dateAdd|dateCompare|dateConvert|dateDiff|dateFormat|datePart|dateTimeFormat|day|dayOfWeek|dayOfWeekAsString|dayOfWeekShortAsString|dayOfYear|daysInMonth|daysInYear|de|decimalFormat|decodeForHTML|decodeFromURL|decrementValue|decrypt|decryptBinary|deleteClientVariable|deserializeJSON|directoryClose|directoryCopy|directoryCreate|directoryDelete|directoryExists|directoryList|directoryRename|dollarFormat|duplicate|each|encodeForCSS|encodeForDN|encodeForHTML|encodeForHTMLAttribute|encodeForJavaScript|encodeForLDAP|encodeForURL|encodeForXML|encodeForXMLAttribute|encodeForXPath|encrypt|encryptBinary|entityDelete|entityLoad|entityLoadByExample|entityLoadByPK|entityMerge|entityNew|entityReload|entitySave|entityToQuery|evaluate|exp|expandPath|fileAppend|fileClose|fileCopy|fileDelete|fileExists|fileGetMimeType|fileIsEOF|fileMove|fileOpen|fileRead|fileReadBinary|fileReadLine|fileSeek|fileSetAccessMode|fileSetAttribute|fileSetLastModified|fileSkipBytes|fileUpload|fileUploadAll|fileWrite|fileWriteLine|find|findNoCase|findOneOf|firstDayOfMonth|fix|floor|formatBaseN|generatePBKDFKey|generateSecretKey|getApplicationMetadata|getApplicationSettings|getAuthUser|getBaseTagData|getBaseTagList|getBaseTemplatePath|getCanonicalPath|getClassInfo|getClientVariablesList|getComponentMetadata|getContextRoot|getCurrentTemplatePath|getDirectoryFromPath|getEncoding|getException|getFileFromPath|getFileInfo|getFunctionCalledName|getFunctionList|getGatewayHelper|getHTTPRequestData|getHTTPTimeString|getK2ServerDocCount|getK2ServerDocCountLimit|getLocale|getLocaleCountry|getLocaleDisplayName|getLocaleInfo|getLocalHostIP|getLocalIP|getMetadata|getMetaData|getNumericDate|getPageContext|getProfileSections|getProfileString|getReadableImageFormats|getSafeHTML|getSOAPRequest|getSOAPRequestHeader|getSOAPResponse|getSOAPResponseHeader|getSystemFreeMemory|getSystemTotalMemory|getTempDirectory|getTempFile|getTickCount|getTimeZoneInfo|getToken|getUserRoles|getVFSMetaData|getWriteableImageFormats|hash|hash40|hmac|hour|htmlCodeFormat|htmlEditFormat|iif|imageAddBorder|imageBlur|imageClearRect|imageCopy|imageCrop|imageDrawArc|imageDrawBeveledRect|imageDrawCubicCurve|imageDrawLine|imageDrawLines|imageDrawOval|imageDrawPoint|imageDrawQuadraticCurve|imageDrawRect|imageDrawRoundRect|imageDrawText|imageFlip|imageGetBlob|imageGetBufferedImage|imageGetEXIFMetadata|imageGetEXIFTag|imageGetHeight|imageGetIPTCMetadata|imageGetIPTCTag|imageGetWidth|imageGrayscale|imageInfo|imageMakeColorTransparent|imageMakeTranslucent|imageNegative|imageNew|imageOverlay|imagePaste|imageRead|imageReadBase64|imageResize|imageRotate|imageRotateDrawingAxis|imageScaleToFit|imageSetAntialiasing|imageSetBackgroundColor|imageSetDrawingColor|imageSetDrawingStroke|imageSetDrawingTransparency|imageShear|imageShearDrawingAxis|imageTranslate|imageTranslateDrawingAxis|imageWrite|imageWriteBase64|imageXORDrawingMode|incrementValue|inputBaseN|insert|int|invalidateOAuthAccessToken|invoke|isArray|isBinary|isBoolean|isClosure|isCustomFunction|isDate|isDDX|isDebugMode|isDefined|isEmpty|isFileObject|isImage|isImageFile|isInstanceOf|isIPv6|isJSON|isK2ServerABroker|isK2ServerDocCountExceeded|isK2ServerOnline|isLeapYear|isLocalHost|isNull|isNumeric|isNumericDate|isObject|isPDFFile|isPDFObject|isQuery|isSimpleValue|isSOAPRequest|isSpreadsheetFile|isSpreadsheetObject|isStruct|isUserInAnyRole|isUserInRole|isUserLoggedIn|isValid|isValidOAuthAccessToken|isWDDX|isXML|isXmlAttribute|isXmlDoc|isXmlElem|isXmlNode|isXmlRoot|isXMLRoot|javacast|jsStringFormat|lCase|left|len|listAppend|listChangeDelims|listCompact|listContains|listContainsNoCase|listDeleteAt|listEach|listEvery|listFilter|listFind|listFindNoCase|listFirst|listGetAt|listInsertAt|listItemTrim|listLast|listLen|listMap|listPrepend|listQualify|listReduce|listReduceRight|listRemoveDuplicates|listRest|listSetAt|listSome|listSort|listToArray|listTrim|listValueCount|listValueCountNoCase|ljustify|location|log|log10|lsDateFormat|lsDateTimeFormat|lsEuroCurrencyFormat|lsIsCurrency|lsIsDate|lsIsNumeric|lsNumberFormat|lsParseCurrency|lsParseDateTime|lsParseEuroCurrency|lsParseNumber|lsTimeFormat|ltrim|max|mid|min|minute|month|monthAsString|monthShortAsString|now|nullValue|numberFormat|objectEquals|objectLoad|objectSave|ORMClearSession|ORMCloseAllSessions|ORMCloseSession|ORMEvictCollection|ORMEvictEntity|ORMEvictQueries|ORMExecuteQuery|ORMFlush|ORMGetSession|ORMGetSessionFactory|ORMReload|paragraphFormat|parameterExists|parseDateTime|parseNumber|pi|precisionEvaluate|preserveSingleQuotes|quarter|queryAddColumn|queryAddRow|queryClose|queryColumnArray|queryColumnCount|queryColumnData|queryColumnExists|queryColumnList|queryConvertForGrid|queryCurrentRow|queryDeleteColumn|queryDeleteRow|queryEach|queryEvery|queryExecute|queryFilter|queryGetCell|queryGetResult|queryGetRow|queryKeyExists|queryMap|queryNew|queryRecordCount|queryReduce|queryRenameColumn|queryRowData|querySetCell|querySlice|querySome|querySort|quotedValueList|rand|randomize|randRange|reEscape|reFind|reFindNoCase|reMatch|reMatchNoCase|releaseComObject|rematch|rematchNoCase|removeChars|removeCachedQuery|repeatString|replace|replaceList|replaceListNoCase|replaceNoCase|reReplace|reReplaceNoCase|restDeleteApplication|restInitApplication|restSetResponse|reverse|right|rjustify|round|rtrim|second|sendGatewayMessage|serializeJSON|sessionGetMetaData|sessionInvalidate|sessionRotate|setEncoding|setLocale|setProfileString|setVariable|sgn|sin|sleep|spanExcluding|spanIncluding|spreadsheetAddAutoFilter|spreadsheetAddColumn|spreadsheetAddFreezePane|spreadsheetAddImage|spreadsheetAddInfo|spreadsheetAddPageBreaks|spreadsheetAddRow|spreadsheetAddRows|spreadsheetAddSplitPane|spreadsheetCreateSheet|spreadsheetDeleteColumn|spreadsheetDeleteColumns|spreadsheetDeleteRow|spreadsheetDeleteRows|spreadsheetFormatCell|spreadsheetFormatCellRange|spreadsheetFormatColumn|spreadsheetFormatColumns|spreadsheetFormatRow|spreadsheetFormatRows|spreadsheetGetCellComment|spreadsheetGetCellFormula|spreadsheetGetCellValue|spreadsheetGetColumnCount|spreadsheetInfo|spreadsheetMergeCells|spreadsheetNew|spreadsheetRead|spreadsheetReadBinary|spreadsheetRemoveSheet|spreadsheetSetActiveSheet|spreadsheetSetActiveSheetNumber|spreadsheetSetCellComment|spreadsheetSetCellFormula|spreadsheetSetCellValue|spreadsheetSetColumnWidth|spreadsheetSetFooter|spreadsheetSetHeader|spreadsheetSetRowHeight|spreadsheetShiftColumns|spreadsheetShiftRows|spreadsheetWrite|sqr|storeAddACL|storeGetACL|storeGetMetadata|storeSetACL|storeSetMetadata|stringEach|stringEvery|stringFilter|stringLen|stringMap|stringReduce|stringReduceRight|stringSome|stringSort|stripCR|structAppend|structClear|structCopy|structCount|structDelete|structEach|structEquals|structEvery|structFilter|structFind|structFindKey|structFindValue|structGet|structInsert|structIsEmpty|structIsCaseSensitive|structKeyArray|structKeyExists|structKeyList|structKeyTranslate|structMap|structNew|structReduce|structSome|structSort|structToSorted|structUpdate|tan|threadJoin|threadTerminate|throw|timeFormat|toBase64|toBinary|toNumeric|toScript|toString|trace|transactionCommit|transactionRollback|transactionSetSavePoint|trim|uCase|ucFirst|urlDecode|urlEncodedFormat|urlSessionFormat|val|valueArray|valueList|verifyClient|week|wrap|writeBody|writeDump|writeLog|writeOutput|xmlChildPos|xmlElemNew|xmlFormat|xmlGetNodeType|xmlNew|xmlParse|xmlSearch|xmlTransform|xmlValidate|year|yesNoFormat)$"))

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: (property_identifier) @function.method))

; Special identifiers
;--------------------

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

([
    (identifier)
    (shorthand_property_identifier)
    (shorthand_property_identifier_pattern)
 ] @constant
 (#match? @constant "^[A-Z_][A-Z\\d_]+$"))

((identifier) @variable.special
 (#match? @variable.special "(?i)^(arguments|variables|local|self|super|this|session|application|request|url|form|cgi|server|cookie|client)$"))

; Literals
;---------

(this) @variable.special
(super) @variable.special
(undefined) @constant.special

[
  (true)
  (false)
  (null)
  (undefined)
] @constant.special

(comment) @comment

[
  (string)
  (template_string)
] @string

(hash_expression
  "#" @punctuation.special)
(hash_empty) @punctuation.special

(regex) @string.regex
(number) @number

; Tokens
;-------

[
  ";"
  (optional_chain)
  (static_chain)
  "."
  ","
] @punctuation.delimiter

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
  "<="
  "<<="
  "="
  "=="
  "==="
  "!="
  "!=="
  "=>"
  ">"
  ">="
  ">>="
  ">>>="
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
  "("
  ")"
  "["
  "]"
  "{"
  "}"
]  @punctuation.bracket

(template_substitution
  "${" @punctuation.special
  "}" @punctuation.special)

[
  "as"
  "break"
  "case"
  "catch"
  "component"
  "const"
  "continue"
  "debugger"
  "default"
  "do"
  "else"
  "export"
  "finally"
  "for"
  "from"
  "function"
  "get"
  "if"
  "import"
  "in"
  "instanceof"
  "let"
  "new"
  "of"
  "query"
  "return"
  "set"
  "static"
  "switch"
  "target"
  "throw"
  "try"
  "var"
  "void"
  "while"
  "with"
] @keyword
