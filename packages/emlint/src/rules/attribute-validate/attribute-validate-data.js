// rule: attribute-validate-data
// -----------------------------------------------------------------------------

import validateUri from "../../util/validateUri";

function attributeValidateData(context, ...opts) {
  return {
    attribute: function(node) {
      console.log(
        `███████████████████████████████████████ attributeValidateData() ███████████████████████████████████████`
      );
      console.log(
        `013 ${`\u001b[${33}m${`opts`}\u001b[${39}m`} = ${JSON.stringify(
          opts,
          null,
          4
        )}`
      );
      console.log(
        `020 attributeValidateData(): node = ${JSON.stringify(node, null, 4)}`
      );

      if (node.attribName === "data") {
        // validate the parent
        if (node.parent.tagName !== "object") {
          context.report({
            ruleId: "attribute-validate-data",
            idxFrom: node.attribStart,
            idxTo: node.attribEnd,
            message: `Tag "${node.parent.tagName}" can't have this attribute.`,
            fix: null
          });
        } else {
          validateUri(node.attribValue, {
            offset: node.attribValueStartAt,
            multipleOK: false
          }).forEach(errorObj => {
            console.log(`038 RAISE ERROR`);
            context.report(
              Object.assign({}, errorObj, {
                ruleId: "attribute-validate-data"
              })
            );
          });
        }
      }
    }
  };
}

export default attributeValidateData;