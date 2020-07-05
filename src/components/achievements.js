import React from "react"
import config from "../../data/appConfig"
import Layout from "./layout"
import styles from "./css/achievement.css"


export default function Achievements() {
    return (
        <Layout>
            <h3><span role="img" aria-label='achivements'>🏆</span> Achievements</h3>
            { config.achievements.map(achievement =>(
               <div className={styles.achievement} key={achievement.id}>
                   <p>

                   <b>{achievement.title}</b><br/>
                   {achievement.subTitle}<br/>
                   {achievement.more}</p>
               </div>
                ))
            }
        </Layout>
    )
}
