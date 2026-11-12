// Draws the app icon (icons/icon-1024.png) and the disk image background (dmg-background.png, 1200x800,
// the 600x400 window at 2x). Run from this folder: swift draw-artwork.swift; `just icons` scales the icon.
import AppKit

func png(_ size: NSSize, _ path: String, _ draw: (NSRect) -> Void) {
    let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(size.width), pixelsHigh: Int(size.height),
                               bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
                               colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0)!
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
    draw(NSRect(origin: .zero, size: size))
    NSGraphicsContext.restoreGraphicsState()
    try! rep.representation(using: .png, properties: [:])!.write(to: URL(fileURLWithPath: path))
}

func symbol(_ name: String, _ points: CGFloat, _ color: NSColor) -> NSImage {
    let config = NSImage.SymbolConfiguration(pointSize: points, weight: .semibold)
        .applying(NSImage.SymbolConfiguration(paletteColors: [color]))
    return NSImage(systemSymbolName: name, accessibilityDescription: nil)!.withSymbolConfiguration(config)!
}

png(NSSize(width: 1024, height: 1024), "icons/icon-1024.png") { rect in
    let tile = NSBezierPath(roundedRect: rect.insetBy(dx: 100, dy: 100), xRadius: 185, yRadius: 185)
    NSGradient(starting: NSColor(red: 0.16, green: 0.42, blue: 0.95, alpha: 1),
               ending: NSColor(red: 0.07, green: 0.20, blue: 0.55, alpha: 1))!.draw(in: tile, angle: -90)
    let glyph = symbol("link", 400, .white)
    let box = NSRect(x: (1024 - glyph.size.width) / 2, y: (1024 - glyph.size.height) / 2,
                     width: glyph.size.width, height: glyph.size.height)
    glyph.draw(in: box)
}

png(NSSize(width: 1200, height: 800), "dmg-background.png") { rect in
    NSColor(red: 0.97, green: 0.97, blue: 0.98, alpha: 1).setFill()
    rect.fill()
    let arrow = symbol("arrow.right", 110, NSColor(white: 0.55, alpha: 1))
    arrow.draw(in: NSRect(x: 600 - arrow.size.width / 2, y: 400 - arrow.size.height / 2,
                          width: arrow.size.width, height: arrow.size.height))
    let text = "Drag http2scheme to Applications" as NSString
    let style = NSMutableParagraphStyle()
    style.alignment = .center
    text.draw(in: NSRect(x: 0, y: 110, width: 1200, height: 70), withAttributes: [
        .font: NSFont.systemFont(ofSize: 44, weight: .medium),
        .foregroundColor: NSColor(white: 0.35, alpha: 1),
        .paragraphStyle: style,
    ])
}
