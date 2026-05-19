import CoreGraphics
import CoreText
import Foundation

typealias CTAdaptiveImageProviderCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    CGSize,
    Double,
    UnsafeMutablePointer<CGPoint>?,
    UnsafeMutablePointer<CGSize>?
) -> UnsafeMutableRawPointer?

typealias CTAdaptiveImageProviderReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

@available(macOS 15.0, *)
final class CTAdaptiveImageProviderBridge: NSObject, CTAdaptiveImageProviding {
    let refcon: UnsafeMutableRawPointer?
    let callback: CTAdaptiveImageProviderCallback
    let releaseCallback: CTAdaptiveImageProviderReleaseCallback

    init(
        refcon: UnsafeMutableRawPointer?,
        callback: @escaping CTAdaptiveImageProviderCallback,
        releaseCallback: @escaping CTAdaptiveImageProviderReleaseCallback
    ) {
        self.refcon = refcon
        self.callback = callback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback(refcon)
    }

    func image(
        forProposedSize proposedSize: CGSize,
        scaleFactor: CGFloat,
        imageOffset outImageOffset: UnsafeMutablePointer<CGPoint>,
        imageSize outImageSize: UnsafeMutablePointer<CGSize>
    ) -> CGImage? {
        guard let rawImage = callback(
            refcon,
            proposedSize,
            Double(scaleFactor),
            outImageOffset,
            outImageSize
        ) else {
            return nil
        }
        return Unmanaged<CGImage>.fromOpaque(rawImage).takeRetainedValue()
    }
}

@_cdecl("ct_adaptive_image_provider_create")
func ct_adaptive_image_provider_create(
    _ refcon: UnsafeMutableRawPointer?,
    _ callback: CTAdaptiveImageProviderCallback?,
    _ releaseCallback: CTAdaptiveImageProviderReleaseCallback?
) -> UnsafeMutableRawPointer? {
    guard let callback, let releaseCallback else {
        return nil
    }
    guard #available(macOS 15.0, *) else {
        return nil
    }
    return Unmanaged.passRetained(
        CTAdaptiveImageProviderBridge(refcon: refcon, callback: callback, releaseCallback: releaseCallback)
    ).toOpaque()
}

@_cdecl("ct_font_get_typographic_bounds_for_adaptive_image_provider")
func ct_font_get_typographic_bounds_for_adaptive_image_provider(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ providerPtr: UnsafeMutableRawPointer?
) -> CGRect {
    guard #available(macOS 15.0, *), let fontPtr else {
        return .zero
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let provider = providerPtr.flatMap {
        Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? CTAdaptiveImageProviding
    }
    return CTFontGetTypographicBoundsForAdaptiveImageProvider(font, provider)
}

@_cdecl("ct_font_draw_image_from_adaptive_image_provider_at_point")
func ct_font_draw_image_from_adaptive_image_provider_at_point(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ providerPtr: UnsafeMutableRawPointer?,
    _ point: CGPoint,
    _ contextPtr: UnsafeMutableRawPointer?
) {
    guard #available(macOS 15.0, *),
          let fontPtr,
          let providerPtr,
          let contextPtr,
          let provider = Unmanaged<AnyObject>.fromOpaque(providerPtr).takeUnretainedValue() as? CTAdaptiveImageProviding
    else {
        return
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let context = Unmanaged<CGContext>.fromOpaque(contextPtr).takeUnretainedValue()
    CTFontDrawImageFromAdaptiveImageProviderAtPoint(font, provider, point, context)
}
