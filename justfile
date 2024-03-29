serve: build
    python3 -m http.server --directory out

build: styles scripts html 

clean:
    rm -rf out out-html out-scripts

styles:
    mkdir -p out/
    nix build -o result-styles .#styles
    rm -f out/*.css out/*.css.map
    cp -r result-styles/styles.css result-styles/styles.css.map out/
    chmod +w -R out/

html:
    mkdir -p out/
    cargo run -- build ./test_data/astrid_dot_tech_example -o out-html --script-assets js
    cp -ar out-html/* out-html/.* out/
    chmod +w -R out/

scripts:
    npx rollup --config
    mkdir -p out/
    rm -rf out/*.js out/*.js.map
    cp -r out-scripts/*.js out-scripts/*.js.map out/
