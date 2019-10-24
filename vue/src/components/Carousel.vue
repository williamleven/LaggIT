<template>
    <section class="carousel">
        <!-- TODO: careousel content -->
        <slot></slot>
        <div class="prev">
            <button @click="prev"><ChevronLeft></ChevronLeft></button>
        </div>
        <div class="next">
            <button @click="next"><ChevronRight></ChevronRight></button>
        </div>
        {{ events }}
    </section>
</template>

<script>
import ChevronLeft from 'vue-material-design-icons/ChevronLeft.vue'
import ChevronRight from 'vue-material-design-icons/ChevronRight.vue'

export default {
  name: 'Carousel',
  components: {
    ChevronLeft,
    ChevronRight
  },
};
</script>

<style lang="scss" scoped>
@import "../assets/theme";

.carousel {
    min-height: 80vh;
    display: grid;
    grid-template-columns: 5% 90% 5%;
    grid-template-rows: 100%;
    overflow: hidden;
    .next, .prev {
        grid-row: 1;
        width: 100%;
        height: 100%;
        background-color: $button-base-color;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all .5s;
        button {
            font-size: 40px;
            color: white;
            border: none;
            cursor: pointer;
            background-color: transparent;
            width: 100%;
            height: 100%;
            padding: 0;
        }
        &:hover {
            background-color: $button-hover-color;
        }
    }
    .next {
        grid-column: 3;
    }
    .prev {
        grid-column: 1;
    }
}
</style>
