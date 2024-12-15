# HTML to RSX Converter for Dioxus

VISIT: https://wheregmis.github.io/dioxus_html_rsx/

## Overview

The HTML to RSX Converter is a specialized Rust tool designed to transform standard HTML markup into Dioxus's RSX (React-like Syntax) format. This utility helps Rust developers using Dioxus to easily migrate existing HTML templates or convert static HTML components into Dioxus-compatible RSX code.

## Features
- 🦀 Pure Rust implementation
- 🔄 Automatic conversion of HTML to Dioxus RSX
- 🧩 Handles complex nested structures
- 📝 Preserves original formatting
- 🚀 Efficient parsing and transformation using native dioxus-rosetta
- 🛡️ Robust error handling

## Prerequisites

- Rust (1.70.0 or higher)
- Dioxus CLI (`dx`)
  ```bash
  cargo install dioxus-cli
  ```

## Running the Project
Clone the repository and do the following
```bash
# Serve the project locally with hot reloading
dx serve --platform web

# Build for production
dx build

# Build for web
dx build --web

# Build for desktop
dx build --desktop
```

## Conversion Rules

The converter applies the following Dioxus-specific transformation rules:

1. Converts `class` to `class`
2. Transforms inline styles to Dioxus style syntax
3. Handles self-closing tags
4. Manages attribute name conversions specific to Dioxus RSX

## Examples

### Before Conversion
```html
<div class="collapse bg-base-200">
  <input type="radio" name="my-accordion-1" checked="checked" />
  <div class="collapse-title text-xl font-medium">Click to open this one and close others</div>
  <div class="collapse-content">
    <p>hello</p>
  </div>
</div>
<div class="collapse bg-base-200">
  <input type="radio" name="my-accordion-1" />
  <div class="collapse-title text-xl font-medium">Click to open this one and close others</div>
  <div class="collapse-content">
    <p>hello</p>
  </div>
</div>
<div class="collapse bg-base-200">
  <input type="radio" name="my-accordion-1" />
  <div class="collapse-title text-xl font-medium">Click to open this one and close others</div>
  <div class="collapse-content">
    <p>hello</p>
  </div>
</div>
```

### After Conversion
```rust
div { class: "collapse bg-base-200",
    input { r#type: "radio", name: "my-accordion-1", checked: "checked" }
    div { class: "collapse-title text-xl font-medium", "Click to open this one and close others" }
    div { class: "collapse-content",
        p { "hello" }
    }
}
div { class: "collapse bg-base-200",
    input { r#type: "radio", name: "my-accordion-1" }
    div { class: "collapse-title text-xl font-medium", "Click to open this one and close others" }
    div { class: "collapse-content",
        p { "hello" }
    }
}
div { class: "collapse bg-base-200",
    input { name: "my-accordion-1", r#type: "radio" }
    div { class: "collapse-title text-xl font-medium", "Click to open this one and close others" }
    div { class: "collapse-content",
        p { "hello" }
    }
}
```

## Contributing

Contributions are welcome! 

### How to Contribute

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Run tests (`cargo test`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

MIT License

## Support

If you encounter any problems or have suggestions, please file an issue on our GitHub repository's issue tracker.

---

**Happy Dioxus Development! 🦀🚀**