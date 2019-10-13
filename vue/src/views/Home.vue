<template>
  <div class="home">
      <section class="left">
          <div>
              <div class="welcome-info">
                  <h1>Välkommen till LaggIT</h1>
                  <p>LaggIT är en rätt cool förening som har LAN och sådant. Ibland har vi Sp(h)eldagar då och då också med en massa andra föreningar.</p>
              </div>
              <a href="#">
                  Bli medlem
              </a>
          </div>
      </section>
      <section class="right">
          <div class="color-overlay">
              <div class="event-box-home">
                  <div class="date-box">
                      <p class="date-month">December</p>
                      <p class="date-number">1 - 3</p>
                      <p> 2034 </p>
                  </div>
                  <div class="event-information-home">
                      <h3> rymdLAN </h3>
                      <div class="small-info">
                          <p>Gratis</p> |
                          <p> Fre: 16:00 - Sön: 12:00</p>
                      </div>
                      <div class="big-info">
                          <p>För att fira första mänskliga utomjordiska befolkningen håller LaggIT ett LAN med rymdtema! Kom och spela Space Invaders!</p>
                      </div>
                  </div>
              </div>
          </div>
      </section>
  </div>
</template>

<script>
export default {
  name: 'home',
}
</script>

<style lang="scss" scoped>
    .home {
        display: grid;
        grid-template-columns: 45% 55%;
        min-height: 80vh;
        .left {
            grid-column: 1;
            height: 100%;
            display: flex;
            justify-content: flex-end;
            align-items: center;
            div {
                display: flex;
                flex-direction: column;
                width: 70%;
                margin-right: 30px;
                .welcome-info {
                    width: 100%;
                    h1 {
                        margin: 10px 0;
                        color: #2b93b8;
                    }
                    p {
                        margin-top: 0;
                        margin-bottom: 20px;
                    }
                }
                a {
                    background-color: #47b5dc;
                    padding: 30px 70px;
                    border-radius: 3px;
                    text-decoration: none;
                    color: black;
                    font-size: 25px;
                    width: 130px;
                    align-self: flex-end;
                    &:hover {
                        background-color: #6fc9ee;
                    }
                }
            }
        }
        .right {
            grid-column: 2;
            height: 100%;
            background-image: url('../assets/rymdLAN.jpg');
            background-position: center;
            background-repeat: no-repeat;
            background-size: cover;
            .color-overlay {
                height: 100%;
                width: 100%;
                display: flex;
                justify-content: center;
                flex-direction: column;
                background-color: rgba(0, 0, 0, 0.5);
            }
            .event-box-home {
                display: flex;
                margin-left: 30px;
                .date-box {
                    display: flex;
                    flex-direction: column;
                    text-align: center;
                    height: 110px;
                    min-width: 100px;
                    padding: 0 10px;
                    background-color: #f6f6f6;
                    border-top: 25px inset black;
                    p {
                        margin: 0;
                    }
                    .date-month {
                        margin-top: 10px;
                    }
                    .date-number {
                        font-size: 45px;
                        margin: 0;
                    }
                }
                .event-information-home {
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    margin-left: 20px;
                    width: 70%;
                    z-index: 100;
                    color: white;
                    h3, p {
                        margin: 0;
                    }
                    .small-info {
                        margin-top: 6px;
                        display: flex;
                        font-weight: bold;
                        p:first-of-type {
                            margin-right: 15px;
                        }
                        p:last-of-type {
                            margin-left: 15px;
                        }
                    }
                    .big-info {
                        margin-top: 6px;
                    }
                }
            }
        }
    }
</style>
