<template>
  <div>
    <input type="text" v-model="value">
    <button @click="addValue">Add</button>
    <div v-for="(item, key) in data" :key="key">
      <span>{{ item }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  data() {
    return {
      value: '',
      data:  []
    }
  },
  beforeCreate() {
    import('../../pkg').then(wasm => {
      this.add = wasm.add;
      this.show = wasm.show;
    });
  },
  methods: {
    addValue() {
      if (this.value.length === 0) {
        return;
      }

      const value = parseInt(this.value);

      this.add(value);
      this.data = this.show();

      this.clearValue();
    },

    clearValue() {
      this.value = '';
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
