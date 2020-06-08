import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"
export default function Contact() {
  return (
    <div >
      <Header></Header>
      <Layout>
        <div style={{ margin: `3rem auto`, maxWidth: 800, padding: `0 1rem` }}>
          <h1>I'd love to talk! Email me for a virtual coffee chat.</h1>
          <p>
            <a href="mailto:hello@karuturirs.com">hello@karuturirs.com</a>
          </p>
        </div>
      </Layout>
    </div>
  )
}