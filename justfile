clean-out:
    rm -rf out

mkstyles:
    mkdir -p out/
    nix build -o result-styles .#styles
    rm -f out/*.css
    cp -r result-styles/styles.css out/

mkhtml:
    mkdir -p out/
    cargo run -- build ./test_data/contentdir_example -o out-html
    cp -r out-html/* out/

serve: clean-out mkstyles mkhtml
    python3 -m http.server --directory out

