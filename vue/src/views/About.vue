<template>
  <div class="about">
    <section class="our-vision">
      <h2>Vår Vision</h2>
      <p> Aliquam at ipsum nisl. Integer at pulvinar ante. In vitae erat quam. Fusce aliquet, justo et fermentum varius, lorem sem mollis est, sit amet sodales odio velit nec dolor. Curabitur pulvinar nunc ac nisl sagittis, nec ornare nisi iaculis. Maecenas fermentum, lorem eu gravida volutpat, nulla ex semper tellus, id venenatis nisl lorem quis lectus. Duis eu risus euismod est molestie cursus. Quisque iaculis, eros ut venenatis condimentum, lorem erat fermentum purus, sed ullamcorper justo diam vel quam. Fusce vulputate, sem et facilisis cursus, velit ante dictum dui, vel mollis ante quam id lorem. </p>
    </section>
    <Carousel>
      <template>
        <div class="the-people-container">
          <h2><span>Styrelsen</span> 23/24</h2>
          <div class="square-grid">
            <article class="person">
              <div class="person-img">
                <img src="../assets/person.png" alt="">
              </div>
              <div class="person-info">
                <h3>Per Persson</h3>
                <p class="nickname">Pixabay</p>
                <em>Ordförande</em>
              </div>
            </article>
          </div>
          <div class="button-container">
            <button> Dokument 1 </button>
            <button> Dokument 2 </button>
          </div>
        </div>
      </template>
    </Carousel>
  </div>
</template>

<script>
import Carousel from '../components/Carousel';

export default {
  name: 'About',
  components: {
    Carousel
  },
}
</script>

<style lang="scss" scoped>
@import "../assets/theme";

.our-vision {
  color: white;
  background-color: $background-color;
  min-height: 50vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  h2 {
    margin-top: 0;
  }
  p {
    text-align: center;
    width: 60%;
    margin: 0;
  }
}

.the-people-container {
  grid-column: 2;
  padding: 30px 50px;
  .button-container {
    display: flex;
    justify-content: space-between;
    button {
      background-color: $button-base-color;
      border: none;
      color: white;
      border-radius: 3px;
      height: 40px;
      font-size: 16px;
      width: 49%;
      transition: all .3s;
      &:hover {
        background-color: $button-hover-color;
        cursor: pointer;
      }
    }
  }
  .square-grid {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    .person {
      width: 200px;
      display: flex;
      flex-direction: column;
      align-items: center;
      margin-bottom: 30px;
      .person-img {
        width: 180px;
        height: 230px;
        overflow: hidden;
      }
      .person-info {
        text-align: center;
        h3 {
          margin: 7px 0 0 0;
        }
        p {
          margin: 0;
          &:first-of-type {
            font-size: 13px;
            margin-bottom: 7px;
          }
        }
        em {
          font-style: normal;
          color: rgb(27, 115, 148);
        }
      }
    }
  }
}
</style>
