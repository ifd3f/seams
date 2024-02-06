{ sass, runCommand }: runCommand "styles" {src=./.; buildInputs = [sass];} ''
  mkdir -p $out
  sass -I=$src $src/main.scss $out/styles.css
''
