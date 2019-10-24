<template>
    <Carousel>
        <template>
            <div class="event first-position">
                <div class="color-overlay">
                    <div class="event-box-home">
                        <div>
                            <div class="date-box">
                                <p class="date-month">December</p>
                                <p class="date-number">1 - 3</p>
                                <p> 2034 </p>
                            </div>
                            <div class="people-at-event">
                                <h4>Anmälda:</h4>
                                <p>71</p>
                            </div>
                        </div>
                        <div class="event-information-home">
                            <h3> rymdLAN </h3>
                            <div class="small-info">
                                <p> Gratis </p> |
                                <p> Fre: 16:00 - Sön: 12:00</p>
                            </div>
                            <div class="big-info">
                                <p>För att fira första mänskliga utomjordiska befolkningen håller LaggIT ett LAN med rymdtema! Kom och spela Space Invaders! Här kommer det
                                    var en längre text för att förklara allt godis, läsk och annat som kommer finnas på evenemanget, woh!</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </template>
    </Carousel>
</template>

<script>
import gql from 'graphql-tag'
import Carousel from '../components/Carousel';

export default {
  name: 'Events',
  components: {
    Carousel
  },
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
    .event {
        height: 100%;
        background-position: center;
        background-repeat: no-repeat;
        background-size: cover;
        background-image: url("../assets/rymdLAN.jpg");
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
            justify-content: center;
            div {
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
                .people-at-event {
                    color: white;
                    text-align: center;
                    margin-top: 15px;
                    h4 {
                        margin: 0;
                        font-weight: normal;
                    }
                    p {
                        margin: 0;
                        font-size: 30px;
                    }
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
    .front-position {
        grid-column: 2;
        grid-row: 1;
    }
    @media only screen and (max-width: 980px) {
        .event {
            .event-box-home {
                flex-direction: column;
                justify-content: center;
                align-items: center;
                margin: 20px 0 50px auto;
                .date-box {
                    max-width: 120px;
                    margin-bottom: 10px;
                }
            }
        }
    }
</style>
