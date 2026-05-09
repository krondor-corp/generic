module.exports = {
  hooks: {
    readPackage(pkg) {
      if (pkg.name === 'libpg-query') {
        // Add a patch script that runs before install
        pkg.scripts = pkg.scripts || {};
        pkg.scripts.preinstall = `
          if [ -f "libpg_query/src/postgres/src_port_snprintf.c" ]; then
            sed -i.bak '374s/^/\\/\\/ /' libpg_query/src/postgres/src_port_snprintf.c
            sed -i.bak 's/strchrnul(format + 1/strchr(format + 1/g' libpg_query/src/postgres/src_port_snprintf.c
          fi
        `;
      }
      return pkg;
    }
  }
};