module.exports = {
  configureWebpack: {
    devServer: {
      proxy: {
        '/graphql': {
          'target': 'http://localhost:8000',
        },
      },
    },
  },
}
