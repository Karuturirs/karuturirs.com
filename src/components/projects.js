import React from "react"
import config from "../../data/appConfig"
import Layout from "./layout"
import styles from "./css/projects.css"

export default function Projects() {
    return (
        <Layout>
            <h3><span role="img" aria-label='Projects'>🚀</span> Projects</h3>
            <div>
                { config.projects.map(project => {
                    return (
                        <a key={project.id} href={project.url}  className="row mouse"
                        target="_blank"
                        rel="noreferrer"> 
                            <div className="cell bold">{project.name}</div>
                            <div className="cell description">{project.title}</div>
                            <div className="cell light">
                                { project.tags.map(tag => (
                                    <a className='tag'>{tag}</a>
                                    ))
                                }
                            </div>
                        </a>
                    )
                })}
            </div>
        </Layout>
    )
}
