(function() {
  'use strict';

  var globals = typeof window === 'undefined' ? global : window;
  if (typeof globals.require === 'function') return;

  var modules = {};
  var cache = {};
  var aliases = {};
  var has = ({}).hasOwnProperty;

  var expRe = /^\.\.?(\/|$)/;
  var expand = function(root, name) {
    var results = [], part;
    var parts = (expRe.test(name) ? root + '/' + name : name).split('/');
    for (var i = 0, length = parts.length; i < length; i++) {
      part = parts[i];
      if (part === '..') {
        results.pop();
      } else if (part !== '.' && part !== '') {
        results.push(part);
      }
    }
    return results.join('/');
  };

  var dirname = function(path) {
    return path.split('/').slice(0, -1).join('/');
  };

  var localRequire = function(path) {
    return function expanded(name) {
      var absolute = expand(dirname(path), name);
      return globals.require(absolute, path);
    };
  };

  var initModule = function(name, definition) {
    var hot = null;
    hot = hmr && hmr.createHot(name);
    var module = {id: name, exports: {}, hot: hot};
    cache[name] = module;
    definition(module.exports, localRequire(name), module);
    return module.exports;
  };

  var expandAlias = function(name) {
    return aliases[name] ? expandAlias(aliases[name]) : name;
  };

  var _resolve = function(name, dep) {
    return expandAlias(expand(dirname(name), dep));
  };

  var require = function(name, loaderPath) {
    if (loaderPath == null) loaderPath = '/';
    var path = expandAlias(name);

    if (has.call(cache, path)) return cache[path].exports;
    if (has.call(modules, path)) return initModule(path, modules[path]);

    throw new Error("Cannot find module '" + name + "' from '" + loaderPath + "'");
  };

  require.alias = function(from, to) {
    aliases[to] = from;
  };

  var extRe = /\.[^.\/]+$/;
  var indexRe = /\/index(\.[^\/]+)?$/;
  var addExtensions = function(bundle) {
    if (extRe.test(bundle)) {
      var alias = bundle.replace(extRe, '');
      if (!has.call(aliases, alias) || aliases[alias].replace(extRe, '') === alias + '/index') {
        aliases[alias] = bundle;
      }
    }

    if (indexRe.test(bundle)) {
      var iAlias = bundle.replace(indexRe, '');
      if (!has.call(aliases, iAlias)) {
        aliases[iAlias] = bundle;
      }
    }
  };

  require.register = require.define = function(bundle, fn) {
    if (typeof bundle === 'object') {
      for (var key in bundle) {
        if (has.call(bundle, key)) {
          require.register(key, bundle[key]);
        }
      }
    } else {
      modules[bundle] = fn;
      delete cache[bundle];
      addExtensions(bundle);
    }
  };

  require.list = function() {
    var list = [];
    for (var item in modules) {
      if (has.call(modules, item)) {
        list.push(item);
      }
    }
    return list;
  };

  var hmr = globals._hmr && new globals._hmr(_resolve, require, modules, cache);
  require._cache = cache;
  require.hmr = hmr && hmr.wrap;
  require.brunch = true;
  globals.require = require;
})();

(function() {
var global = window;
var process;
var __makeRelativeRequire = function(require, mappings, pref) {
  var none = {};
  var tryReq = function(name, pref) {
    var val;
    try {
      val = require(pref + '/node_modules/' + name);
      return val;
    } catch (e) {
      if (e.toString().indexOf('Cannot find module') === -1) {
        throw e;
      }

      if (pref.indexOf('node_modules') !== -1) {
        var s = pref.split('/');
        var i = s.lastIndexOf('node_modules');
        var newPref = s.slice(0, i).join('/');
        return tryReq(name, newPref);
      }
    }
    return none;
  };
  return function(name) {
    if (name in mappings) name = mappings[name];
    if (!name) return;
    if (name[0] !== '.' && pref) {
      var val = tryReq(name, pref);
      if (val !== none) return val;
    }
    return require(name);
  }
};
require.register("components/app.vue", function(exports, require, module) {
;(function(){
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});

var _toolbar = require("./toolbar.vue");

var _toolbar2 = _interopRequireDefault(_toolbar);

var _translationGroup = require("./translation-group.vue");

var _translationGroup2 = _interopRequireDefault(_translationGroup);

var _navigationBar = require("./navigation-bar.vue");

var _navigationBar2 = _interopRequireDefault(_navigationBar);

var _lodash = require("lodash");

var _lodash2 = _interopRequireDefault(_lodash);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

exports.default = {
  name: 'app',

  props: {
    store: Object
  },

  computed: {
    translationKeys: function translationKeys() {
      return _lodash2.default.keys(this.store.groupedTranslations);
    }
  },

  components: {
    toolbar: _toolbar2.default,
    translationGroup: _translationGroup2.default,
    navigationBar: _navigationBar2.default
  }
};
})()
if (module.exports.__esModule) module.exports = module.exports.default
var __vue__options__ = (typeof module.exports === "function"? module.exports.options: module.exports)
if (__vue__options__.functional) {console.error("[vueify] functional components are not supported and should be defined in plain js files using render functions.")}
__vue__options__.render = function(){with(this){return _h('div',{attrs:{"id":"main"}},[_h('toolbar')," ",_h('div',{attrs:{"id":"workspace"}},[_h('navigation-bar',{attrs:{"translation-keys":translationKeys}})," ",_h('div',{attrs:{"id":"translation-list"}},[_l((store.groupedTranslations),function(translations,key){return _h('translation-group',{attrs:{"translation-key":key,"translations":translations}})})])])])}}
__vue__options__.staticRenderFns = []
if (module.hot) {(function () {  var hotAPI = require("vue-hot-reload-api")
  hotAPI.install(require("vue"), true)
  if (!hotAPI.compatible) return
  module.hot.accept()
  if (!module.hot.data) {
    hotAPI.createRecord("data-v-1", __vue__options__)
  } else {
    hotAPI.reload("data-v-1", __vue__options__)
  }
})()}
});

;require.register("components/navigation-bar.vue", function(exports, require, module) {
;(function(){
'use strict';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = {
  name: 'navigation-bar',

  props: {
    translationKeys: Array
  }
};
})()
if (module.exports.__esModule) module.exports = module.exports.default
var __vue__options__ = (typeof module.exports === "function"? module.exports.options: module.exports)
if (__vue__options__.functional) {console.error("[vueify] functional components are not supported and should be defined in plain js files using render functions.")}
__vue__options__.render = function(){with(this){return _h('div',{attrs:{"id":"navigation-bar"}},[_h('ul',{staticClass:"nav-translation-key-list"},[_l((translationKeys),function(key){return _h('li',{staticClass:"nav-translation-key"},[_h('a',{attrs:{"href":"#"},on:{"click":function($event){$event.preventDefault();}}},[_m(0,true),"\n        "+_s(key)+"\n        ",_m(1,true)])])})])])}}
__vue__options__.staticRenderFns = [function(){with(this){return _h('i',{staticClass:"fa fa-fw fa-file-o"})}},function(){with(this){return _h('i',{staticClass:"fa fa-fw fa-chevron-right"})}}]
if (module.hot) {(function () {  var hotAPI = require("vue-hot-reload-api")
  hotAPI.install(require("vue"), true)
  if (!hotAPI.compatible) return
  module.hot.accept()
  if (!module.hot.data) {
    hotAPI.createRecord("data-v-2", __vue__options__)
  } else {
    hotAPI.reload("data-v-2", __vue__options__)
  }
})()}
});

;require.register("components/toolbar.vue", function(exports, require, module) {
;(function(){
'use strict';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = {
  name: 'toolbar'
};
})()
if (module.exports.__esModule) module.exports = module.exports.default
var __vue__options__ = (typeof module.exports === "function"? module.exports.options: module.exports)
if (__vue__options__.functional) {console.error("[vueify] functional components are not supported and should be defined in plain js files using render functions.")}
__vue__options__.render = function(){with(this){return _m(0)}}
__vue__options__.staticRenderFns = [function(){with(this){return _h('div',{attrs:{"id":"toolbar"}},[_h('h1',["Lugh"])])}}]
if (module.hot) {(function () {  var hotAPI = require("vue-hot-reload-api")
  hotAPI.install(require("vue"), true)
  if (!hotAPI.compatible) return
  module.hot.accept()
  if (!module.hot.data) {
    hotAPI.createRecord("data-v-3", __vue__options__)
  } else {
    hotAPI.reload("data-v-3", __vue__options__)
  }
})()}
});

;require.register("components/translation-group.vue", function(exports, require, module) {
;(function(){
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});

var _translationKey = require("./translation-key.vue");

var _translationKey2 = _interopRequireDefault(_translationKey);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

exports.default = {
  name: 'translation-group',

  props: {
    translations: Array,
    translationKey: String
  },

  components: {
    translationKey: _translationKey2.default
  }
};
})()
if (module.exports.__esModule) module.exports = module.exports.default
var __vue__options__ = (typeof module.exports === "function"? module.exports.options: module.exports)
if (__vue__options__.functional) {console.error("[vueify] functional components are not supported and should be defined in plain js files using render functions.")}
__vue__options__.render = function(){with(this){return _h('div',{staticClass:"translation-group"},[_h('translation-key',{attrs:{"translation-key":translationKey}})," ",_l((translations),function(translation){return _h('div',{staticClass:"translation",attrs:{"t-id":translation.id}},[_h('div',{staticClass:"translation-locale"},["\n      "+_s(translation.locale)+"\n    "])," ",_h('textarea',[_s(translation.content)])])})])}}
__vue__options__.staticRenderFns = []
if (module.hot) {(function () {  var hotAPI = require("vue-hot-reload-api")
  hotAPI.install(require("vue"), true)
  if (!hotAPI.compatible) return
  module.hot.accept()
  if (!module.hot.data) {
    hotAPI.createRecord("data-v-4", __vue__options__)
  } else {
    hotAPI.reload("data-v-4", __vue__options__)
  }
})()}
});

;require.register("components/translation-key.vue", function(exports, require, module) {
;(function(){
'use strict';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = {
  name: 'translation-key',

  props: {
    translationKey: String
  },

  computed: {
    crumbs: function crumbs() {
      return this.translationKey.split('.');
    }
  }
};
})()
if (module.exports.__esModule) module.exports = module.exports.default
var __vue__options__ = (typeof module.exports === "function"? module.exports.options: module.exports)
if (__vue__options__.functional) {console.error("[vueify] functional components are not supported and should be defined in plain js files using render functions.")}
__vue__options__.render = function(){with(this){return _h('div',{staticClass:"translation-key"},[_l((crumbs),function(crumb,index){return _h('span',{staticClass:"translation-crumb"},[(index > 0)?_h('i',{staticClass:"fa fa-fw fa-caret-right translation-crumb-separator"},[_m(0,true)]):_e(),_s(crumb)])})])}}
__vue__options__.staticRenderFns = [function(){with(this){return _h('span',["."])}}]
if (module.hot) {(function () {  var hotAPI = require("vue-hot-reload-api")
  hotAPI.install(require("vue"), true)
  if (!hotAPI.compatible) return
  module.hot.accept()
  if (!module.hot.data) {
    hotAPI.createRecord("data-v-5", __vue__options__)
  } else {
    hotAPI.reload("data-v-5", __vue__options__)
  }
})()}
});

;require.register("start.js", function(exports, require, module) {
'use strict';

var _vue = require('vue');

var _vue2 = _interopRequireDefault(_vue);

var _app = require('components/app.vue');

var _app2 = _interopRequireDefault(_app);

var _lodash = require('lodash');

var _lodash2 = _interopRequireDefault(_lodash);

require('whatwg-fetch');

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var store = {
  groupedTranslations: []
};

window.vue = new _vue2.default({
  el: '#app',
  data: store,
  render: function render(createElement) {
    return createElement('app', {
      props: {
        store: this.$data
      }
    });
  },

  components: {
    'app': _app2.default
  }
});

fetch("/api/translations").then(function (response) {
  return response.json();
}).then(function (data) {
  store.groupedTranslations = _lodash2.default.groupBy(data, 'key');
});
});

require.alias("buffer/index.js", "buffer");
require.alias("process/browser.js", "process");process = require('process');require.register("___globals___", function(exports, require, module) {
  
});})();require('___globals___');


//# sourceMappingURL=app.js.map