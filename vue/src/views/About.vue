<template>
  <div class="about">
    <section class="our-vision">
      <h2>Vår Vision</h2>
      <p> Aliquam at ipsum nisl. Integer at pulvinar ante. In vitae erat quam. Fusce aliquet, justo et fermentum varius, lorem sem mollis est, sit amet sodales odio velit nec dolor. Curabitur pulvinar nunc ac nisl sagittis, nec ornare nisi iaculis. Maecenas fermentum, lorem eu gravida volutpat, nulla ex semper tellus, id venenatis nisl lorem quis lectus. Duis eu risus euismod est molestie cursus. Quisque iaculis, eros ut venenatis condimentum, lorem erat fermentum purus, sed ullamcorper justo diam vel quam. Fusce vulputate, sem et facilisis cursus, velit ante dictum dui, vel mollis ante quam id lorem. </p>
    </section>
    <section class="the-people">
      <div class="the-people-container">
        <h2><span>Styrelsen</span> 23/24</h2>
        <div class="square-grid">
          <div class="person">
            <div class="person-img">
              <img src="../assets/person.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per Persson</h3>
              <p class="nickname">Pixabay</p>
              <em>Ordförande</em>
            </div>
          </div>

          <div class="person">
            <div class="person-img">
              <img src="../assets/person2.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per-Lisa Perlisasson</h3>
              <p>Pixachu</p>
              <em>Kassör</em>
            </div>
          </div>

          <div class="person">
            <div class="person-img">
              <img src="../assets/person2.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per-Lisa Perlisasson</h3>
              <p>Pixachu</p>
              <em>Kassör</em>
            </div>
          </div>

          <div class="person">
            <div class="person-img">
              <img src="../assets/person2.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per-Lisa Perlisasson</h3>
              <p>Pixachu</p>
              <em>Kassör</em>
            </div>
          </div>

          <div class="person">
            <div class="person-img">
              <img src="../assets/person2.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per-Lisa Perlisasson</h3>
              <p>Pixachu</p>
              <em>Kassör</em>
            </div>
          </div>

          <div class="person">
            <div class="person-img">
              <img src="../assets/person2.png" alt="">
            </div>
            <div class="person-info">
              <h3>Per-Lisa Perlisasson</h3>
              <p>Pixachu</p>
              <em>Kassör</em>
            </div>
          </div>
          
        </div>
        <div class="button-container">
          <button> Dokument 1 </button>
          <button> Dokument 2 </button>
        </div>
      </div>
      <div class="prev">
        <button @click="prev">Föregående</button>
      </div>
      <div class="next">
        <button @click="next">Nästa</button>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'About',
}
</script>

<style lang="scss" scoped>
.our-vision {
  color: white;
  background-color: #125d78;
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
.the-people {
  min-height: 50vh;
  display: grid;
  grid-template-columns: 5% 90% 5%;
  grid-template-rows: 100%;
  .the-people-container {
    grid-column: 2;
    padding: 30px 50px;
    .button-container {
      display: flex;
      justify-content: space-between;
      button {
        background-color: #074055;
        border: none;
        color: white;
        border-radius: 3px;
        height: 40px;
        font-size: 16px;
        width: 49%;

        &:hover {
          background-color: #052c3a;
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
          img {

          }
        }
        .person-info {
          text-align: center;
          h3 {
            margin: 7px 0 0 0;
          }
          p {
            margin: 0;
          }
          p:first-of-type {
            font-size: 13px;
            margin-bottom: 7px;
          }
          em {
            font-style: normal;
            color: rgb(27, 115, 148);
          }
        }
      }
    }
  }
  .next, .prev {
    grid-row: 1;
    width: 100%;
    height: 100%;
    background-color: #074055;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .next {
    grid-column: 3;
    button {
      font-size: 20px;
      color: white;
      border: none;
      cursor: pointer;
      transform: rotate(-90deg);
      background-color: transparent;
    }
  }
  .prev {
    grid-column: 1;
    button {
      font-size: 20px;
      color: white;
      border: none;
      cursor: pointer;
      transform: rotate(90deg);
      background-color: transparent;
    }
  }
}
</style>
