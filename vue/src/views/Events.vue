<template>
    <div class="events">
        <!-- TODO: Event carousel -->
      <button @click="prev">Prev</button>
      <button @click="next">Next</button>
      {{ events }}
    </div>
</template>

<script>
import gql from 'graphql-tag'

export default {
  name: 'Events',
  data () {
    return {
      index: 1,
    }
  },
  methods: {
    next () {
      this.index++
    },
    prev () {
      this.index--
    },
  },
  apollo: {
    events: {
      query: gql`
        query ($high: Int!, $low: Int!) {
          events(high: $high, low: $low) {
            id
            title
            startTime
            endTime
            background
            location
            price
            signupCount
          }
        }
      `,
      variables () {
        return {
          high: this.index + 1,
          low: this.index - 1,
        }
      },
    },
  },
}
</script>

<style lang="scss" scoped>

</style>
