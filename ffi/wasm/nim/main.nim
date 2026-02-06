## 🔥 NIM WASM MODULE - Real HTML Parsing & Content Extraction Engine
##
## Compiled with Nim for WebAssembly:
##   nim c -d:release -d:emscripten --os:linux --cpu:wasm32 --gc:arc -o:nim_parser.wasm main.nim
##
## Or with native Emscripten:
##   nim c -d:release -d:emscripten --passC:"-s WASM=1" --passL:"-s EXPORTED_FUNCTIONS=['_parse_html','_extract_links','_extract_metadata','_alloc','_dealloc']" main.nim
##
## Exports:
##   - parse_html(html_ptr, html_len, selector_ptr, selector_len) -> result_ptr
##   - extract_links(html_ptr, html_len) -> result_ptr
##   - extract_metadata(html_ptr, html_len) -> result_ptr
##   - count_words(text_ptr, text_len) -> word_count
##   - detect_language(text_ptr, text_len) -> lang_id
##   - alloc(size) -> ptr
##   - dealloc(ptr, size)
##
## Powers MCP Tools: websearch, premium, scan, osint_intelligence

import std/[strutils, json, sequtils, tables, unicode]

# ═══════════════════════════════════════════════════════════════════════════
# MEMORY MANAGEMENT - Exported for WASM host
# ═══════════════════════════════════════════════════════════════════════════

proc allocMem(size: cint): pointer {.exportc: "alloc".} =
  result = alloc(size)

proc deallocMem(p: pointer, size: cint) {.exportc: "dealloc".} =
  dealloc(p)

proc writeResult(data: string): cint =
  let buf = allocMem(cint(data.len + 1))
  if buf == nil:
    return 0
  copyMem(buf, unsafeAddr data[0], data.len)
  cast[ptr UncheckedArray[byte]](buf)[data.len] = 0
  return cint(cast[int](buf))

proc readMemory(p: cint, length: cint): string =
  result = newString(length)
  let src = cast[ptr UncheckedArray[byte]](cast[pointer](p))
  for i in 0..<length:
    result[i] = char(src[i])

# ═══════════════════════════════════════════════════════════════════════════
# HTML ELEMENT TYPES
# ═══════════════════════════════════════════════════════════════════════════

type
  HtmlElement = object
    tag: string
    text: string
    attributes: Table[string, string]
  
  HtmlLink = object
    url: string
    text: string
    title: string
    rel: string

  HtmlMetadata = object
    title: string
    description: string
    keywords: seq[string]
    author: string
    language: string
    charset: string
    ogTitle: string
    ogDescription: string
    ogImage: string

# ═══════════════════════════════════════════════════════════════════════════
# HTML PARSER ENGINE - Powers: websearch, premium, osint_intelligence
# ═══════════════════════════════════════════════════════════════════════════

proc findTagContent(html: string, tag: string): seq[HtmlElement] =
  ## Extract all elements of a given tag type
  result = @[]
  let openTag = "<" & tag
  let closeTag = "</" & tag & ">"
  var pos = 0
  
  while pos < html.len:
    let start = html.find(openTag, pos)
    if start < 0: break
    
    # Find end of opening tag
    let tagEnd = html.find('>', start)
    if tagEnd < 0: break
    
    # Extract attributes from opening tag
    let tagStr = html[start + 1 .. tagEnd - 1]
    var attrs = initTable[string, string]()
    
    # Parse attributes
    var attrStr = tagStr
    let spaceIdx = attrStr.find(' ')
    if spaceIdx >= 0:
      attrStr = attrStr[spaceIdx + 1 .. ^1]
      # Simple attribute parser
      var i = 0
      while i < attrStr.len:
        # Skip whitespace
        while i < attrStr.len and attrStr[i] == ' ': inc i
        if i >= attrStr.len: break
        
        # Find attribute name
        let nameStart = i
        while i < attrStr.len and attrStr[i] != '=' and attrStr[i] != ' ': inc i
        if i <= nameStart: break
        let name = attrStr[nameStart .. i - 1].strip()
        
        # Find value
        if i < attrStr.len and attrStr[i] == '=':
          inc i
          if i < attrStr.len and (attrStr[i] == '"' or attrStr[i] == '\''):
            let quote = attrStr[i]
            inc i
            let valStart = i
            while i < attrStr.len and attrStr[i] != quote: inc i
            if i > valStart:
              attrs[name] = attrStr[valStart .. i - 1]
            inc i
          else:
            let valStart = i
            while i < attrStr.len and attrStr[i] != ' ': inc i
            if i > valStart:
              attrs[name] = attrStr[valStart .. i - 1]
        else:
          attrs[name] = ""
    
    # Find content between tags
    let contentStart = tagEnd + 1
    let contentEnd = html.find(closeTag, contentStart)
    
    var textContent = ""
    if contentEnd >= 0:
      textContent = html[contentStart .. contentEnd - 1]
      # Strip nested HTML tags for text content
      var clean = ""
      var inTag = false
      for ch in textContent:
        if ch == '<': inTag = true
        elif ch == '>': inTag = false
        elif not inTag: clean.add(ch)
      textContent = clean.strip()
      pos = contentEnd + closeTag.len
    else:
      pos = tagEnd + 1
    
    result.add(HtmlElement(
      tag: tag,
      text: textContent,
      attributes: attrs
    ))

proc matchesSelector(element: HtmlElement, selector: string): bool =
  ## Check if element matches a CSS-like selector
  if selector.startsWith("."):
    let className = selector[1..^1]
    return element.attributes.getOrDefault("class", "").contains(className)
  elif selector.startsWith("#"):
    let id = selector[1..^1]
    return element.attributes.getOrDefault("id", "") == id
  elif selector.contains("["):
    let parts = selector.split("[")
    let tagName = parts[0]
    if tagName.len > 0 and element.tag != tagName:
      return false
    if parts.len > 1:
      let attrPart = parts[1].replace("]", "")
      if attrPart.contains("="):
        let attrParts = attrPart.split("=")
        let attrName = attrParts[0]
        let attrVal = attrParts[1].replace("\"", "").replace("'", "")
        return element.attributes.getOrDefault(attrName, "") == attrVal
      else:
        return element.attributes.hasKey(attrPart)
    return true
  else:
    return element.tag == selector

# ═══════════════════════════════════════════════════════════════════════════
# EXPORTED WASM FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════

proc parseHtml(htmlPtr: cint, htmlLen: cint, selectorPtr: cint, selectorLen: cint): cint {.exportc: "parse_html".} =
  ## Parse HTML and extract elements matching selector
  let html = readMemory(htmlPtr, htmlLen)
  let selector = readMemory(selectorPtr, selectorLen)
  
  # Determine which tags to search based on selector
  var searchTags: seq[string] = @[]
  if selector.startsWith(".") or selector.startsWith("#"):
    searchTags = @["div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6",
                   "a", "section", "article", "main", "header", "footer",
                   "nav", "aside", "ul", "ol", "li", "table", "tr", "td"]
  elif selector.contains("["):
    let parts = selector.split("[")
    if parts[0].len > 0:
      searchTags = @[parts[0]]
    else:
      searchTags = @["a", "img", "meta", "link", "input", "script"]
  else:
    searchTags = @[selector]
  
  var results: seq[JsonNode] = @[]
  
  for tag in searchTags:
    let elements = findTagContent(html, tag)
    for elem in elements:
      if matchesSelector(elem, selector):
        var attrs = newJObject()
        for key, value in elem.attributes:
          attrs[key] = newJString(value)
        
        results.add(%*{
          "tag": elem.tag,
          "text": elem.text,
          "attributes": attrs
        })
  
  let resultJson = $(%results)
  return writeResult(resultJson)

proc extractLinks(htmlPtr: cint, htmlLen: cint): cint {.exportc: "extract_links".} =
  ## Extract all links from HTML
  let html = readMemory(htmlPtr, htmlLen)
  let elements = findTagContent(html, "a")
  
  var links: seq[JsonNode] = @[]
  for elem in elements:
    links.add(%*{
      "url": elem.attributes.getOrDefault("href", ""),
      "text": elem.text,
      "title": elem.attributes.getOrDefault("title", ""),
      "rel": elem.attributes.getOrDefault("rel", "")
    })
  
  let resultJson = $(%links)
  return writeResult(resultJson)

proc extractMetadata(htmlPtr: cint, htmlLen: cint): cint {.exportc: "extract_metadata".} =
  ## Extract page metadata from HTML
  let html = readMemory(htmlPtr, htmlLen)
  
  # Extract title
  var title = ""
  let titleElements = findTagContent(html, "title")
  if titleElements.len > 0:
    title = titleElements[0].text
  
  # Extract meta tags
  var description = ""
  var keywords: seq[string] = @[]
  var author = ""
  var ogTitle = ""
  var ogDescription = ""
  var ogImage = ""
  
  let metaElements = findTagContent(html, "meta")
  for elem in metaElements:
    let name = elem.attributes.getOrDefault("name", "")
    let property = elem.attributes.getOrDefault("property", "")
    let content = elem.attributes.getOrDefault("content", "")
    
    if name == "description": description = content
    elif name == "keywords": keywords = content.split(",").mapIt(it.strip())
    elif name == "author": author = content
    elif property == "og:title": ogTitle = content
    elif property == "og:description": ogDescription = content
    elif property == "og:image": ogImage = content
  
  # Detect language
  var language = "unknown"
  let lowerHtml = html.toLowerAscii()
  if lowerHtml.contains("lang=\"en") or lowerHtml.contains("lang='en"):
    language = "english"
  elif lowerHtml.contains("lang=\"es") or lowerHtml.contains("lang='es"):
    language = "spanish"
  elif lowerHtml.contains("lang=\"fr") or lowerHtml.contains("lang='fr"):
    language = "french"
  elif lowerHtml.contains("lang=\"de") or lowerHtml.contains("lang='de"):
    language = "german"
  
  let result = %*{
    "title": title,
    "description": description,
    "keywords": keywords,
    "author": author,
    "language": language,
    "og_title": ogTitle,
    "og_description": ogDescription,
    "og_image": ogImage
  }
  
  return writeResult($result)

proc countWords(textPtr: cint, textLen: cint): cint {.exportc: "count_words".} =
  ## Count words in text using Unicode-aware splitting
  let text = readMemory(textPtr, textLen)
  var count: cint = 0
  var inWord = false
  
  for ch in text:
    if ch == ' ' or ch == '\n' or ch == '\t' or ch == '\r':
      if inWord:
        inc count
        inWord = false
    else:
      inWord = true
  
  if inWord:
    inc count
  
  return count

proc detectLanguage(textPtr: cint, textLen: cint): cint {.exportc: "detect_language".} =
  ## Detect language from text content
  ## Returns: 0=unknown, 1=english, 2=spanish, 3=french, 4=german, 5=portuguese
  let text = readMemory(textPtr, textLen).toLowerAscii()
  
  # English markers
  let enMarkers = ["the ", " and ", " is ", " are ", " was ", " were ", " have ", " has "]
  var enScore = 0
  for marker in enMarkers:
    enScore += text.count(marker)
  
  # Spanish markers
  let esMarkers = [" el ", " la ", " los ", " las ", " es ", " son ", " con ", " que "]
  var esScore = 0
  for marker in esMarkers:
    esScore += text.count(marker)
  
  # French markers
  let frMarkers = [" le ", " la ", " les ", " des ", " est ", " sont ", " avec ", " que "]
  var frScore = 0
  for marker in frMarkers:
    frScore += text.count(marker)
  
  # German markers
  let deMarkers = [" der ", " die ", " das ", " und ", " ist ", " sind ", " mit ", " von "]
  var deScore = 0
  for marker in deMarkers:
    deScore += text.count(marker)
  
  # Portuguese markers
  let ptMarkers = [" o ", " a ", " os ", " as ", " é ", " são ", " com ", " que "]
  var ptScore = 0
  for marker in ptMarkers:
    ptScore += text.count(marker)
  
  let maxScore = max([enScore, esScore, frScore, deScore, ptScore])
  if maxScore == 0: return 0
  if enScore == maxScore: return 1
  if esScore == maxScore: return 2
  if frScore == maxScore: return 3
  if deScore == maxScore: return 4
  if ptScore == maxScore: return 5
  return 0
