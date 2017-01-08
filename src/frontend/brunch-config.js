module.exports = {
  files: {
    javascripts: {
      joinTo: {
        'vendor.js': /^(?!app)/,
        'app.js': /^app/
      }
    },
    stylesheets: {joinTo: 'app.css'}
  },

  plugins: {
    copycat: {
      "fonts": [
        "node_modules/font-awesome/fonts",
        "node_modules/font-awesome/css"
      ],
      "flags": [
        "node_modules/flag-icon-css/flags",
        "node_modules/flag-icon-css/css"
      ],
      "onlyChanged": true,
      "verbose": false
    }
  }
};
