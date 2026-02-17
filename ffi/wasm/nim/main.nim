import std/[json, strutils, xmltree, htmlparser, streams, tables]

# WASM Memory Management

proc alloc(size: int): pointer {.exportc.} =
  result = alloc0(size)

proc dealloc(ptr: pointer, size: int) {.exportc.} =
  dealloc(ptr)

# ═══════════════════════════════════════════════════════════════════════════
# HTML PARSING & EXTRACTION - Powers: websearch, premium, osint_intelligence
# ═══════════════════════════════════════════════════════════════════════════

type
  HtmlElement = object
    tag: string
    text: string
    attributes: Table[string, string]
    children: seq[HtmlElement]

proc parseHtmlNode(node: XmlNode): HtmlElement =
  result.tag = node.tag
  result.text = node.innerText
  result.attributes = initTable[string, string]()
  if node.attrs != nil:
    for key, value in node.attrs:
      result.attributes[key] = value
  result.children = @[]
  for child in node:
    result.children.add(parseHtmlNode(child))

proc parse_html(html_ptr: pointer, html_len: int, selector_ptr: pointer, selector_len: int): pointer {.exportc.} =
  let html = cast[string](html_ptr)[0 ..< html_len]
  let selector = cast[string](selector_ptr)[0 ..< selector_len]

  try:
    let root = parseHtml(newStringStream(html))
    # Simple selector implementation (tag name only for now)
    var elements: seq[HtmlElement] = @[]

    proc traverse(node: XmlNode) =
      if node.tag == selector:
        elements.add(parseHtmlNode(node))
      for child in node:
        traverse(child)

    traverse(root)

    let jsonStr = $ %elements
    let resLen = jsonStr.len
    let resPtr = alloc(resLen + 1)
    copyMem(resPtr, jsonStr[0].addr, resLen)
    return resPtr
  except:
    let err = "{\"error\": \"Parsing failed\"}"
    let resPtr = alloc(err.len + 1)
    copyMem(resPtr, err[0].addr, err.len)
    return resPtr

# ═══════════════════════════════════════════════════════════════════════════
# METADATA EXTRACTION
# ═══════════════════════════════════════════════════════════════════════════

proc extract_metadata(html_ptr: pointer, html_len: int): pointer {.exportc.} =
  let html = cast[string](html_ptr)[0 ..< html_len]
  var metadata = initTable[string, string]()
  
  try:
    let root = parseHtml(newStringStream(html))
    
    proc findMeta(node: XmlNode) =
      if node.tag == "meta":
        var name = ""
        var content = ""
        if node.attrs != nil:
          if node.attrs.hasKey("name"): name = node.attrs["name"]
          elif node.attrs.hasKey("property"): name = node.attrs["property"]

          if node.attrs.hasKey("content"): content = node.attrs["content"]

          if name != "" and content != "":
            metadata[name] = content

      if node.tag == "title":
        metadata["title"] = node.innerText
        
      for child in node:
        findMeta(child)
        
    findMeta(root)
    
    let jsonStr = $ %metadata
    let resLen = jsonStr.len
    let resPtr = alloc(resLen + 1)
    copyMem(resPtr, jsonStr[0].addr, resLen)
    return resPtr
  except:
    return nil

# ═══════════════════════════════════════════════════════════════════════════
# LINK EXTRACTION
# ═══════════════════════════════════════════════════════════════════════════

proc extract_links(html_ptr: pointer, html_len: int): pointer {.exportc.} =
  let html = cast[string](html_ptr)[0 ..< html_len]
  var links: seq[string] = @[]
  
  try:
    let root = parseHtml(newStringStream(html))

    proc findLinks(node: XmlNode) =
      if node.tag == "a":
        if node.attrs != nil and node.attrs.hasKey("href"):
          links.add(node.attrs["href"])
      for child in node:
        findLinks(child)
        
    findLinks(root)
    
    let jsonStr = $ %links
    let resLen = jsonStr.len
    let resPtr = alloc(resLen + 1)
    copyMem(resPtr, jsonStr[0].addr, resLen)
    return resPtr
  except:
    return nil

# ═══════════════════════════════════════════════════════════════════════════
# UTILS
# ═══════════════════════════════════════════════════════════════════════════

proc count_words(text_ptr: pointer, text_len: int): int {.exportc.} =
  let text = cast[string](text_ptr)[0 ..< text_len]
  return text.splitWhitespace().len

proc detect_language(text_ptr: pointer, text_len: int): pointer {.exportc.} =
  # Simple heuristic: check common words
  let text = cast[string](text_ptr)[0 ..< text_len].toLowerAscii()
  var lang = "unknown"
  
  if "the " in text or " and " in text: lang = "en"
  elif "el " in text or " la " in text: lang = "es"
  elif "le " in text or " la " in text: lang = "fr" # ambiguous but simple
  elif "der " in text or " die " in text: lang = "de"
  
  let resLen = lang.len
  let resPtr = alloc(resLen + 1)
  copyMem(resPtr, lang[0].addr, resLen)
  return resPtr
