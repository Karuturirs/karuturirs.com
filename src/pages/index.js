import React from "react"
import { Link, graphql } from "gatsby"
import { css } from "@emotion/core"
import { rhythm } from "../utils/typography"
import Header from "../components/header"
import Layout from "../components/layout"
import SEO from "../components/seo"
import Achivements from "../components/achievements"
import Projects from "../components/projects"

export default function Home({ data }) {
  return (
    <div>
      <SEO title="Homepage Karuturirs.com" description=" Articles | Tech discussions | Ideas and Innovation" />
      <Header>
      </Header>
      <Achivements/>
      <Projects/>
      <Layout>
          <h3> <span role="img" aria-label='achivements'>📝</span> Latest Works</h3>
          <h4>{data.allMarkdownRemark.totalCount} Posts</h4>
          {data.allMarkdownRemark.edges.map(({ node }) => (
            <div key={node.id}  className="mouse" style={{padding: `10px`}}>
              <Link
                to={node.fields.slug}
                css={css`
                  text-decoration: none;
                  color: inherit;
                `}
              >
                <h3
                  css={css`
                    margin-bottom: ${rhythm(1 / 4)};
                  `}
                >
                  {node.frontmatter.title}{" "}
                  <span
                    css={css`
                      color: #bbb;
                    `}
                  >
                    — {node.frontmatter.date}
                  </span>
                </h3>
                <p>{node.excerpt}</p>
              </Link>
            </div>
          ))}
      </Layout>
    </div>
  )
}

export const query = graphql`
  query {
    allMarkdownRemark(sort: { fields: [frontmatter___date], order: DESC }) {
      totalCount
      edges {
        node {
          id
          frontmatter {
            title
            date(formatString: "DD MMMM, YYYY")
          }
          fields {
            slug
          }
          excerpt
        }
      }
    }
  }
`
