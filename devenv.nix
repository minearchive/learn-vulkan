{
  pkgs,
  lib,
  ...
}:
{
  env.LD_LIBRARY_PATH = lib.makeLibraryPath [
    pkgs.vulkan-loader
  ];

  env.SHADERC_LIB_DIR = "${pkgs.shaderc.lib}/lib";

  packages = [
    pkgs.git
    pkgs.vulkan-tools
    pkgs.vulkan-headers
    pkgs.vulkan-loader
    pkgs.shaderc
    pkgs.cmake
    pkgs.clang
    pkgs.pkg-config
    pkgs.xorg.libxcb
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
  ];
  languages.rust.enable = true;
}
