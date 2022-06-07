# MemeDb
Building, testing, and running dev servers is done through `npm`.

## Repo initialization
Fairly sure you just need the Tauri prerequisites installed before you can build and run the dev servers.

## Development server

To build both processes and run the two servers run the following command. This will automatically open the GUI on the root page. 
```
npm run tauri dev
```

## Building release

To build a release version the following command can be run. This will produce a `.msi` file in `./src-tauri/target/release/bundle/msi/`.
```
npm run tauri build
```

## Issues

Please check out the issues 