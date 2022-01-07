# New Custom Language

Meant as a "wrapper" or "layer" over-top of Rust. More accessible, faster development, just as safe (if not safer), but less powerful, and less performant.

Developed by Adam Bates.

---

- `let` - Declares a new variable that is allowed to be reassigned

```
let x = 123;
x = 456;
```

- `const` - Declares a new variable that is **not** allowed to be reassigned

```
const x = 123;
```

- `&...`

- `"..."`

- `[...]`

- `(...)`

- `..`

- `if`

- `match`

- `loop`

- `while`

- `for`

- `define`

- `structure`

- `state`

- `function`

- `mutable` - Marks data as mutable, allowing mutation. Also can be used to force structures to always be mutable.

```
define DeviceModel = mutable structure {
  state {
    public const name: string,
  }
}

define Device = mutable structure {
  state {
    public const serial: string,
    public let model: mutable DeviceModel,
  }
}

function update_device_serial(device: mutable Device, serial: string) {
  device.serial = serial;
}

function main() {
  const model = mutable DeviceModel {
    name: "MODEL_1",
  };

  const device = mutable Device {
    serial: "ABC123",
    model,
  }

  update_device_name(mutable device, "DEF456");
}
```
