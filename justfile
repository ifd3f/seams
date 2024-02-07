serve: clean-out styles html
    python3 -m http.server --directory out

clean-out:
    rm -rf out

styles:
    mkdir -p out/
    nix build -o result-styles .#styles
    rm -f out/*.css
    rm -f out/*.css.map
    cp -r result-styles/styles.css out/
    cp -r result-styles/styles.css.map out/

html:
    mkdir -p out/
    cargo run -- build ./test_data/contentdir_example -o out-html
    cp -r out-html/* out/

