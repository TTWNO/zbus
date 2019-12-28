# zbus

A Rust API for D-Bus communication. The aim is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C libraries.

The project is divided into two crates:

## zvariant

This crate provides API for encoding/decoding of data to/from D-Bus wire format. This crate is already in good shape
and can and should be used by other projects. This wire format is simple and very efficient binary format and hence
useful outside of D-Bus context as well.

## zbus

That's the main crate that you'll use to actually communicate with services and apps over D-Bus. At the moment you can
only connect to the session bus and call methods synchronously.

# Status

## zvariant

Stable.

## zbus

Early development. You've been warned!

# License

MIT license [LICENSE-MIT](LICENSE-MIT)

# Dependencies

  * nix
  * byteorder

Don't be impressed. I'm sure this list will grow soon. :)
