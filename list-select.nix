{ stdenv
, fetchgit
, rustPlatform_1_41
}:

rustPlatform_1_41.buildRustPackage rec {
  pname   = "list-select";
  version = "0.1.0";

  src = fetchgit {
    url = ./.;
    rev = "aa344a8";
    sha256 = "1ws83910vksgjm5yqznqxind56d6v9dhd3czwwwkp18ia0vccs5k";
  };
  cargoSha256 = "1yig68hv8p9xl2s4s7bkca5wm5qkq1wk08hp2whr48maxig53vy4";

  meta = with stdenv.lib; {
    description = "TUI to select a vertical list entry";
    homepage    = "https://github.com/thibran/list-select";
    license     = licenses.bsd2;
    platforms   = platforms.unix;
  };
}
