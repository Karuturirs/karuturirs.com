import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"
import SEO from "../components/seo"

export default function Contact() {
  return (
    <div >
      <SEO title= "karuturirs.com contact"  description="Like to talk on any topics Email me." />
      <Header></Header>
      <Layout>
          <h3>I'd love to talk! Email me for a virtual coffee chat.</h3>
          <p>
            <a href="mailto:hello@karuturirs.com">hello@karuturirs.com</a>
          </p>
      </Layout>
    </div>
  )
}