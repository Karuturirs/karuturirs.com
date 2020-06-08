import React from "react"
import Header from "../components/header"
import Layout from "../components/layout"

export default function Home() {
  return (
    <div>
    <Header>
    </Header>
    <Layout>
      <div style={{ margin: `3rem auto`, maxWidth: 800, padding: `0 1rem` }}>
        <h1>Amazing Pandas Eating Things</h1>
        <div>
          <img
            src="https://2.bp.blogspot.com/-BMP2l6Hwvp4/TiAxeGx4CTI/AAAAAAAAD_M/XlC_mY3SoEw/s1600/panda-group-eating-bamboo.jpg"
            alt="Group of pandas eating bamboo"
          />
        </div>
      </div>
    </Layout>
    </div>
  )
}
