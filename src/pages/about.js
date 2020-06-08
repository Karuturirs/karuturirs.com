import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"

export default function About() {
  return (
    <div >
      <Header></Header>
      <Layout>
          <h3>About me</h3>
          <p>
            I’m good enough, I’m smart enough, and gosh darn it, people like me!
          </p>
      </Layout>
    </div>
  )
}