import React from "react"
import Header from "../components/header"
import Layout from "../components/layout"

export default function Home() {
  return (
    <div>
    <Header>
    </Header>
    <Layout>
        <h3>Amazing Pandas Eating Things</h3>
        <div>
          <img
            src="https://2.bp.blogspot.com/-BMP2l6Hwvp4/TiAxeGx4CTI/AAAAAAAAD_M/XlC_mY3SoEw/s1600/panda-group-eating-bamboo.jpg"
            alt="Group of pandas eating bamboo"
          />
        </div>
    </Layout>
    </div>
  )
}
