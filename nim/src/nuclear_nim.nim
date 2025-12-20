import htmlparser
import xmltree
import strutils
import sequtils

type
  NimHtmlResult* = object
    title*: cstring
    content*: cstring
    links*: cstring
    images*: cstring
    error*: cstring

proc parse_html*(html: cstring, base_url: cstring): NimHtmlResult {.exportc, dynlib.} =
  try:
    let html_str = $html
    let base = $base_url

    # Parse HTML
    let doc = parseHtml(html_str)

    # Extract title
    var title = ""
    let titleNodes = doc.findAll("title")
    if titleNodes.len > 0:
      title = titleNodes[0].innerText

    # Extract main content (simplified)
    var content = ""
    let bodyNodes = doc.findAll("body")
    if bodyNodes.len > 0:
      content = bodyNodes[0].innerText

    # Extract links
    var links = ""
    for link in doc.findAll("a"):
      let href = link.attr("href")
      if href != "":
        links &= href & "\n"

    # Extract images
    var images = ""
    for img in doc.findAll("img"):
      let src = img.attr("src")
      if src != "":
        images &= src & "\n"

    result = NimHtmlResult(
      title: cstring(title),
      content: cstring(content),
      links: cstring(links),
      images: cstring(images),
      error: nil
    )

  except Exception as e:
    result = NimHtmlResult(
      title: nil,
      content: nil,
      links: nil,
      images: nil,
      error: cstring(e.msg)
    )
