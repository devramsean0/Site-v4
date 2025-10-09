import { bunStimulusPlugin } from "bun-stimulus-plugin";

await Bun.build({
    entrypoints: ["src/js/index.ts"],
    outdir: "compiled_assets/js",
    plugins: [bunStimulusPlugin()],
    minify: true,
    env: "PUBLIC_*"
})