import { defineConfig } from 'umi';

export default defineConfig({
  nodeModulesTransform: {
    type: 'none',
  },
  antd: {},
  dynamicImport: {},
  hash: true,
  fastRefresh: {},
  routes: [
    { path: '/', component: '@/pages/index' },
  ],
});
