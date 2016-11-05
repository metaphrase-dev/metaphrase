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
    babel: {presets: ['es2015']},
    copycat: {
      "fonts": [
        "node_modules/font-awesome/fonts",
        "node_modules/font-awesome/css"
      ],
      "onlyChanged": true,
      "verbose": false
    }
  }
};
