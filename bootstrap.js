import './style.scss';
import 'element-ui/lib/theme-chalk/index.css';

import("./pkg").then(module => {
  module.run_app();
});
