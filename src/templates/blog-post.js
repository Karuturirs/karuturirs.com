import React from "react"
import { graphql } from "gatsby"
import Layout from "../components/layout"
import Header from "../components/header"
import SEO from "../components/seo"
import style from "./blog-post.css"

export default function BlogPost({data}) {
  const post = data.markdownRemark
  return (
   <div>
      <SEO title={post.frontmatter.title} description={post.excerpt} />
      <Header></Header>
      <Layout>
          <div>
            <h1>{post.frontmatter.title}</h1>
            <div>
                <p className={style.blogTag}>
                    <span className="readtime">{post.frontmatter.readTime}</span>
                    <span className="tags">{post.frontmatter.tags}</span>
                </p>
            </div>

            <div dangerouslySetInnerHTML={{ __html: post.html }} />
          </div>
      </Layout>
   </div>
  )
}

export const query = graphql`
  query($slug: String!) {
    markdownRemark(fields: { slug: { eq: $slug } }) {
      html
      frontmatter {
        date
        title
        tags
        readTime
      }
      excerpt
    }
  }
`