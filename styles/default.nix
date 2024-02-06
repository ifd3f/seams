{ sass, runCommand }: runCommand "styles" {src=./.; buildInputs = [sass];} ''
  mkdir -p $out
  sass --sourcemap=inline $src/index.scss $out/styles.css
''
