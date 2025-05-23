# DesignTime

DesignTime is a UI language/template engine written in Rust that allows you to define UIs in a language similar to JSX.

## Syntax

```tsx
import { Checkbox } from "@designtime.core.ui.MUI"

page Home {
    layout: Glassmorphism
    render: { 
        <div class="container">
            <h1>Welcome to DesignTime</h1>
            <Checkbox checked={true}>Do you see this? {1+1}</Checkbox>
        </div>
    }
    functions: {
        onSelect: () => {
            let x = 40;
            let y = 2;
            let result = x + y;
            return result;
        }
    }
}
```

## Future
- Compile to various target languages
- VM with vite-like server
- External UI components (MUI, ShadCN, etc.)

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
### Open Arnold Development License (Apache 2.0 + Commons Clause)

[View the full license here](./LICENSE)


